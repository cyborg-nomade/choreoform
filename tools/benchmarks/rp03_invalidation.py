# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Apply real missing obligations to this finite assembler's explicit data clauses.

This recognizes only the assembler's known line/block layout, not general source
editing. It preserves every old link and rejects inconsistent fixture structure.
"""
from collections import defaultdict
from authoring_probe import lex
from .rp01_source import quote


def apply_missing(source, ledger, body, missing):
    declarations={r['id']:r for r in ledger['declarations']}
    slots={(r['scope'],r['kind'],r['name']):r['id'] for r in declarations.values()}
    additions=defaultdict(list)
    for row in missing:
        cell,site=row['cell'],row['site']
        assert site not in body['data'][cell]['invalidates']
        node=declarations[site];assert node['kind']=='node'
        chain=[];scope=node['scope']
        while scope is not None:
            r=declarations[scope];assert r['kind']=='scope'
            chain.append(r['name']);scope=r['scope']
        additions[cell].append((site,quote(node['name'])+''.join(' in '+quote(n) for n in chain)))
    stack=[];out=[];seen=set();lines=source.splitlines(keepends=True);index=0
    while index<len(lines):
        line=lines[index]
        if line.startswith('Process '):
            tokens=lex(line);root=ledger['root'];assert tokens[1][1]==declarations[root]['name']
            assert not stack;stack.append(root)
        elif line.startswith('Scope '):
            tokens=lex(line);assert tokens[3][1]==declarations[stack[-1]]['name']
            stack.append(slots[stack[-1],'scope',tokens[1][1]])
        elif line.startswith(('End scope.','End process.')):
            stack.pop()
        elif line.startswith('Keep '):
            name=lex(line)[1][1];cell=slots[stack[-1],'data',name]
            assert cell not in seen;seen.add(cell)
            block=[line]
            while not block[-1].rstrip().endswith('End data.'):
                index+=1;block.append(lines[index])
            text=''.join(block)
            if cell in additions:
                assert text.count('Invalidate ')==1 and text.count('. Initially ')==1
                before,rest=text.split('Invalidate ',1);old,after=rest.split('. Initially ',1)
                assert old=='empty' or (old.startswith('[') and old.endswith(']'))
                extra=', '.join(ref for _,ref in sorted(additions.pop(cell)))
                content=extra if old=='empty' else old[1:-1]+', '+extra
                text=before+'Invalidate ['+content+']. Initially '+after
            out.append(text);index+=1;continue
        out.append(line);index+=1
    assert not stack and not additions
    assert seen=={r['id'] for r in declarations.values() if r['kind']=='data'}
    return ''.join(out)
