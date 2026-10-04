# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Bounded specification oracle. Not a compiler, validator or editing product.

The recognizer interprets the checked-in EBNF rather than duplicating its rules.
Ledger checks are deliberately independent of source/IR semantic admission.
"""

from functools import lru_cache
from hashlib import sha256
from copy import deepcopy
import json
from pathlib import Path
import re
import unicodedata

import jsonschema
import rfc8785

ROOT = Path(__file__).resolve().parents[1]
MAX_BYTES = 1024 * 1024
MAX_TOKENS = 4096
MAX_STATES = 100000
MAX_DEPTH = 64


def reject(message):
    raise ValueError(message)


def quoted(source, start):
    """Decode the two supported escapes without Unicode normalization."""
    value = []
    i = start + 1
    while i < len(source):
        c = source[i]
        i += 1
        if c == '"':
            return ''.join(value), i
        if c == '\\':
            if i == len(source) or source[i] not in ('"', '\\'):
                reject("unsupported string escape")
            c = source[i]
            i += 1
        if unicodedata.category(c) in {"Cc", "Cs", "Zl", "Zp"}:
            reject("control or surrogate in string")
        value.append(c)
    reject("unterminated string")


def lex(source):
    """Return (kind, value) tokens. Spans/lowering are outside this oracle."""
    if len(source.encode('utf-8')) > MAX_BYTES or source.startswith('\ufeff'):
        reject("source byte limit or BOM")
    tokens = []
    i = depth = 0
    while i < len(source):
        c = source[i]
        if c in ' \t\r\n':
            i += 1
            continue
        if c == '#':
            end = source.find('\n', i)
            i = len(source) if end == -1 else end + 1
            continue
        if c == '"':
            value, i = quoted(source, i)
            tokens.append(('TEXT', value))
        elif c in '[](),.;':
            tokens.append(('PUNCT', c))
            depth += (1 if c in '[(' else -1 if c in '])' else 0)
            if depth < 0 or depth > MAX_DEPTH:
                reject("delimiter depth limit")
            i += 1
        else:
            match = re.match(r'-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?|[A-Za-z]+(?:-[A-Za-z]+)*', source[i:])
            if match is None:
                reject("unsupported token")
            value = match[0]
            i += len(value)
            if value[0] in '-0123456789' and i < len(source) and (source[i].isalnum() or source[i] in '_-'):
                reject("numeric token requires a boundary")
            tokens.append(('NUMBER' if value[0] in '-0123456789' else 'WORD', value))
        if len(tokens) > MAX_TOKENS:
            reject("oracle token limit")
    if depth:
        reject("unclosed delimiter")
    return tuple(tokens)


def load_grammar(path=ROOT / 'docs/authoring/candidate-a.ebnf'):
    """Parse this project's small EBNF metasyntax; reject unknown references."""
    source = '\n'.join(line.split('#', 1)[0] for line in path.read_text().splitlines())
    parts = re.findall(r'"[^"\n]*"|[A-Za-z_][A-Za-z_0-9]*|[=;|()*+?]', source)
    if re.sub(r'\s+', '', source) != ''.join(parts):
        reject("unsupported EBNF metasyntax")
    i = 0
    rules = {}

    def choice():
        nonlocal i
        options = [sequence()]
        while parts[i] == '|':
            i += 1
            options.append(sequence())
        return ('choice', tuple(options))

    def sequence():
        nonlocal i
        items = []
        while parts[i] not in ('|', ')', ';'):
            token = parts[i]
            i += 1
            if token == '(':
                item = choice()
                if parts[i] != ')':
                    reject("EBNF closing parenthesis")
                i += 1
            elif token.startswith('"'):
                item = ('terminal', token[1:-1])
            elif re.fullmatch(r'[A-Za-z_][A-Za-z_0-9]*', token):
                item = ('ref', token)
            else:
                reject("EBNF atom")
            if parts[i] in ('*', '+', '?'):
                item = (parts[i], item)
                i += 1
            items.append(item)
        if not items:
            reject("empty EBNF sequence")
        return ('sequence', tuple(items))

    try:
        while i < len(parts):
            name = parts[i]
            if name in rules or parts[i + 1] != '=':
                reject("duplicate or invalid EBNF rule")
            i += 2
            rules[name] = choice()
            if parts[i] != ';':
                reject("EBNF missing semicolon")
            i += 1
    except IndexError as error:
        raise ValueError("truncated EBNF") from error

    def references(node):
        if node[0] == 'ref':
            return {node[1]}
        if node[0] in ('sequence', 'choice'):
            return set().union(*(references(child) for child in node[1]))
        if node[0] in ('*', '+', '?'):
            return references(node[1])
        return set()

    known = set(rules) | {'NAME', 'TEXT', 'INTEGER', 'NUMBER', 'NAT'}
    if set().union(*(references(rule) for rule in rules.values())) - known:
        reject("undefined EBNF reference")
    return rules


def recognize(source, start='program', rules=None):
    """All-alternative bounded recognition, never first-match language semantics."""
    tokens = lex(source)
    if start == 'program' and (len(tokens) < 3 or tokens[2] != ('TEXT', '0.2.0')):
        reject("unsupported source profile")
    rules = load_grammar() if rules is None else rules
    if start not in rules:
        reject("unknown grammar start rule")
    states = 0

    @lru_cache(maxsize=None)
    def walk(node, position):
        nonlocal states
        states += 1
        if states > MAX_STATES:
            reject("oracle state limit")
        kind, value = node
        if kind == 'ref' and value in rules:
            return walk(rules[value], position)
        if kind == 'terminal' or kind == 'ref':
            if position == len(tokens):
                return frozenset()
            token_kind, token = tokens[position]
            matches = token_kind in {'WORD', 'PUNCT'} and kind == 'terminal' and token == value
            if kind == 'ref':
                matches = ((value == 'TEXT' and token_kind == 'TEXT')
                           or (value == 'NAME' and token_kind == 'TEXT' and bool(token))
                           or (value == 'NUMBER' and token_kind == 'NUMBER')
                           or (value == 'INTEGER' and token_kind == 'NUMBER' and re.fullmatch(r'0|-?[1-9][0-9]*', token))
                           or (value == 'NAT' and token_kind == 'NUMBER' and re.fullmatch(r'0|[1-9][0-9]*', token)))
            return frozenset({position + 1}) if matches else frozenset()
        if kind == 'choice':
            return frozenset().union(*(walk(child, position) for child in value))
        if kind == 'sequence':
            positions = frozenset({position})
            for child in value:
                positions = frozenset().union(*(walk(child, p) for p in positions))
                if not positions:
                    break
            return positions
        if kind == '?':
            return walk(value, position) | {position}
        reached = set(walk(value, position)) if kind == '+' else {position}
        frontier = reached.copy()
        while frontier:
            next_positions = set().union(*(walk(value, p) for p in frontier)) - reached
            if any(p <= min(frontier) for p in next_positions):
                reject("non-consuming EBNF repetition")
            reached.update(next_positions)
            frontier = next_positions
        return frozenset(reached)

    try:
        if len(tokens) not in walk(('ref', start), 0):
            reject("source does not match grammar")
    except RecursionError as error:
        raise ValueError("oracle recursion limit") from error
    return states


def exact_decimal(spelling, scale):
    """Literal coefficient conversion only; never floating point."""
    if type(scale) is not int or not 0 <= scale <= 18 or not re.fullmatch(r'-?(0|[1-9][0-9]*)(\.[0-9]+)?', spelling):
        reject("invalid decimal spelling or scale")
    fraction = spelling.partition('.')[2]
    if len(fraction) != scale or (scale == 0 and '.' in spelling):
        reject("decimal scale mismatch")
    coefficient = int(spelling.replace('.', ''))
    if (coefficient == 0 and spelling.startswith('-')) or not -(2**63) <= coefficient < 2**63:
        reject("decimal negative zero or coefficient overflow")
    return str(coefficient)


def strict_json(raw):
    if len(raw) > MAX_BYTES or raw.startswith(b'\xef\xbb\xbf'):
        reject("companion byte limit or BOM")

    def pairs(items):
        result = {}
        for key, value in items:
            if key in result:
                reject("duplicate JSON key")
            result[key] = value
        return result

    # All companion numbers occur in annotations. Use the same transport bounds
    # as IR, even though annotation values do not affect its semantic revision.
    def integer(spelling):
        value = int(spelling)
        if spelling == '-0' or abs(value) > 9007199254740991:
            reject("invalid JSON integer")
        return value

    try:
        value = json.loads(raw.decode('utf-8'), object_pairs_hook=pairs,
                           parse_int=integer, parse_float=lambda _: reject("JSON float"),
                           parse_constant=lambda _: reject("JSON non-finite number"))
    except (RecursionError, UnicodeError) as error:
        raise ValueError("invalid JSON nesting or Unicode") from error

    def bounded(item, depth):
        if depth > MAX_DEPTH:
            reject("JSON nesting limit")
        if isinstance(item, str):
            item.encode('utf-8')  # Reject unpaired surrogate escapes.
        elif isinstance(item, dict):
            for key, child in item.items():
                bounded(key, depth + 1)
                bounded(child, depth + 1)
        elif isinstance(item, list):
            for child in item:
                bounded(child, depth + 1)

    try:
        bounded(value, 0)
    except (RecursionError, UnicodeError) as error:
        raise ValueError("invalid JSON nesting or Unicode") from error
    return value


def validate_ledger(ledger):
    schema = json.loads((ROOT / 'schemas/authoring/bindings-0.1.schema.json').read_text())
    jsonschema.Draft202012Validator(schema).validate(ledger)
    declarations = {row['id']: row for row in ledger['declarations']}
    root = declarations.get(ledger['root'])
    if root is None or root['kind'] != 'scope' or root['scope'] is not None:
        reject("invalid root scope")
    active = [ledger['definition']]
    active.extend(row['id'] for group in ('declarations', 'generated', 'symbols') for row in ledger[group])
    if len(active) != len(set(active)) or set(active) & set(ledger['retired']):
        reject("duplicate or retired active identity")
    ancestry = {}
    for row in declarations.values():
        if row['id'] == ledger['root']:
            continue
        chain = []
        parent = row['scope']
        while parent is not None:
            if parent == row['id'] or parent in chain or parent not in declarations or declarations[parent]['kind'] != 'scope':
                reject("invalid lexical scope tree")
            chain.append(parent)
            if len(chain) > MAX_DEPTH:
                reject("scope depth limit")
            parent = declarations[parent]['scope']
        if not chain or chain[-1] != ledger['root']:
            reject("disconnected declaration")
        ancestry[row['id']] = chain
    slots = set()
    declared_names = {(row['scope'], row['name']): row['id'] for row in declarations.values()}
    for row in declarations.values():
        slot = (row['scope'], row['name'])
        if slot in slots:
            reject("duplicate declaration name")
        slots.add(slot)
        # The root's process name is a qualification label, not a local symbol.
        for ancestor in ancestry.get(row['id'], []):
            other = declared_names.get((ancestor, row['name']))
            if other is not None and other != row['id']:
                reject("ancestor shadowing")
    symbol_ids = {row['id']: row for row in ledger['symbols']}
    slots = set()
    for row in symbol_ids.values():
        owner = declarations.get(row['owner'])
        allowed = {'outcome': {'scope', 'node'}, 'branch': {'node'}, 'input': {'scope'}, 'output': {'scope'}, 'purpose': {'data'}}
        if owner is None or owner['kind'] not in allowed[row['role']]:
            reject("invalid symbol owner")
        for category in ('name', 'wire'):
            slot = (row['owner'], row['role'], category, row[category])
            if slot in slots:
                reject("duplicate symbol alias or wire token")
            slots.add(slot)
    slots = set()
    for row in ledger['generated']:
        owner = declarations.get(row['owner'])
        key = row['key']
        role = row['role']
        if owner is None:
            reject("missing generated owner")
        valid = ((role == 'initial' and owner['kind'] == 'data' and key is None)
                 or (role == 'provider-policy' and owner['kind'] == 'capability' and key is None)
                 or (role == 'assignment' and owner['kind'] == 'node' and key in declarations and declarations[key]['kind'] == 'data')
                 or (role == 'flow' and owner['kind'] == 'node' and key in symbol_ids and symbol_ids[key]['owner'] == owner['id'] and symbol_ids[key]['role'] == 'outcome'))
        slot = (row['owner'], role, key)
        if not valid or slot in slots:
            reject("invalid or duplicate generated slot")
        slots.add(slot)
    resource_ids = [row['binding'] for row in ledger['resources']]
    if len(resource_ids) != len(set(resource_ids)) or set(resource_ids) != {key for key, row in declarations.items() if row['kind'] == 'resource'}:
        reject("resource bindings must have exactly one pin")
    return ledger


def resolve(ledger, scope, name, kind, qualifiers=()):
    """Exact lexical lookup; qualifiers must name the actual outward chain."""
    validate_ledger(ledger)
    rows = {row['id']: row for row in ledger['declarations']}
    if scope not in rows or rows[scope]['kind'] != 'scope':
        reject("unknown lookup scope")
    while scope is not None:
        candidates = [row for row in rows.values() if row['scope'] == scope and row['name'] == name]
        if candidates:
            row = candidates[0]
            outward = []
            owner = scope
            while owner is not None:
                outward.append(rows[owner]['name'])
                owner = rows[owner]['scope']
            if qualifiers and tuple(outward) != tuple(qualifiers):
                reject("qualification must specify the full owning chain")
            if row['kind'] != kind:
                reject("wrong declaration kind")
            return row['id']
        scope = rows[scope]['scope']
    reject("unbound exact name")


def package_digests(source, companion):
    """Exact source, JCS ledger, length-framed joint integrity; no semantic hash."""
    source.decode('utf-8')
    if len(source) > MAX_BYTES or source.startswith(b'\xef\xbb\xbf'):
        reject("source byte limit or BOM")
    ledger = validate_ledger(strict_json(companion))
    canonical = rfc8785.dumps(ledger)
    if len(canonical) > MAX_BYTES:
        reject("canonical companion byte limit")
    framed = b'choreoform-authoring-package:0.2.0\0'
    framed += len(source).to_bytes(8, 'big') + source
    framed += len(canonical).to_bytes(8, 'big') + canonical
    return {key: 'sha256:' + sha256(value).hexdigest() for key, value in
            (('source', source), ('bindings', canonical), ('package', framed))}


def rename_ledger(ledger, identity, new_name):
    """Test model for the ledger half of a rename; does not rewrite source."""
    validate_ledger(ledger)
    changed = deepcopy(ledger)
    rows = [row for group in ('declarations', 'symbols') for row in changed[group]
            if row['id'] == identity]
    if len(rows) != 1:
        reject("unknown rename identity")
    rows[0]['name'] = new_name
    validate_ledger(changed)
    return changed


def verify_resources(ledger, supplied):
    """Hash exact caller-supplied bytes. No path discovery or network access."""
    validate_ledger(ledger)
    for pin in list(ledger['contracts'].values()) + ledger['resources']:
        raw = supplied.get((pin['contract'], pin['digest']))
        if raw is None or 'sha256:' + sha256(raw).hexdigest() != pin['digest']:
            reject("missing or mismatched resource bytes")


def join_tokens(ledger, population, selected):
    """Model lossless lowering of a qualified population-wide outcome set."""
    validate_ledger(ledger)
    scopes = {row['id'] for row in ledger['declarations'] if row['kind'] == 'scope'}
    if not population or not set(population) <= scopes or not selected:
        reject("invalid join population or empty outcome set")
    symbols = {row['id']: row for row in ledger['symbols']}
    if any(key not in symbols or symbols[key]['role'] != 'outcome' or symbols[key]['owner'] not in population for key in selected):
        reject("join outcome outside paired population")
    tokens = {symbols[key]['wire'] for key in selected}
    represented = {key for key, row in symbols.items()
                   if row['role'] == 'outcome' and row['owner'] in population and row['wire'] in tokens}
    if set(selected) != represented:
        reject("join token also matches an unlisted member outcome")
    return tokens


def symbol_wire(ledger, owner, role, name):
    """Resolve an alias to its persistent wire spelling, never a generated slug."""
    validate_ledger(ledger)
    matches = [row for row in ledger['symbols'] if
               (row['owner'], row['role'], row['name']) == (owner, role, name)]
    if len(matches) != 1:
        reject("unbound exact symbol alias")
    return matches[0]['wire']
