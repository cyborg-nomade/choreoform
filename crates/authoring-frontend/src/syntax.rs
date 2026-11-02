// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

use crate::{MAX_BYTES, MAX_DEPTH, MAX_TOKENS, MAX_WORK, Result, error};
use std::{collections::BTreeMap, ops::Range, sync::OnceLock};

const GRAMMAR: &str = include_str!("../../../docs/authoring/candidate-a.ebnf");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Text,
    Number,
    Word,
    Punctuation,
}

#[derive(Debug)]
struct Token {
    kind: Kind,
    value: String,
    span: Range<usize>,
}

/// A named production with token extents, not an executable/semantic tree.
/// Lexical classes are leaves. Literal grammar punctuation is omitted from children.
#[derive(Debug)]
pub struct Tree {
    pub rule: &'static str,
    pub(crate) tokens: Range<usize>,
    pub children: Vec<Tree>,
}

impl Tree {
    pub(crate) fn all(&self, rule: &str) -> Vec<&Tree> {
        self.children.iter().filter(|n| n.rule == rule).collect()
    }
    pub(crate) fn one(&self, rule: &str) -> Result<&Tree> {
        let found = self.all(rule);
        if found.len() != 1 {
            return Err(error("internal-tree", 0..0));
        }
        Ok(found[0])
    }
    pub(crate) fn first(&self) -> Result<&Tree> {
        self.children
            .first()
            .ok_or_else(|| error("internal-tree", 0..0))
    }
}

/// Original source, immutable tokens and production tree with exact byte extents.
#[derive(Debug)]
pub struct Syntax<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    tree: Tree,
}

impl Syntax<'_> {
    pub fn source(&self) -> &str {
        self.source
    }
    pub fn tree(&self) -> &Tree {
        &self.tree
    }
    pub fn span(&self, tree: &Tree) -> Range<usize> {
        let start = self
            .tokens
            .get(tree.tokens.start)
            .map_or(self.source.len(), |t| t.span.start);
        let end = tree
            .tokens
            .end
            .checked_sub(1)
            .and_then(|n| self.tokens.get(n))
            .map_or(start, |t| t.span.end);
        start..end
    }
    pub(crate) fn word(&self, tree: &Tree, offset: usize) -> Result<&str> {
        self.tokens
            .get(tree.tokens.start + offset)
            .filter(|_| tree.tokens.start + offset < tree.tokens.end)
            .map(|t| t.value.as_str())
            .ok_or_else(|| error("internal-tree", self.span(tree)))
    }
    pub(crate) fn text(&self, tree: &Tree) -> Result<&str> {
        self.word(tree, 0)
    }
    /// Stable, deliberately simple token formatter. Comments are not restored.
    /// Original verbatim source remains available through `source()`.
    pub fn formatted(&self) -> Result<String> {
        let mut output = String::new();
        for token in &self.tokens {
            let word = if token.kind == Kind::Text {
                format!(
                    "\"{}\"",
                    token.value.replace('\\', "\\\\").replace('"', "\\\"")
                )
            } else {
                token.value.clone()
            };
            if output.len() + word.len() + 1 > MAX_BYTES {
                return Err(error("size", token.span.clone()));
            }
            output.push_str(&word);
            output.push(if token.value == "." && token.kind == Kind::Punctuation {
                '\n'
            } else {
                ' '
            });
        }
        Ok(output)
    }
}

fn lex(source: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut delimiters = Vec::new();
    while i < source.len() {
        let start = i;
        let c = source[i..].chars().next().expect("nonempty UTF-8 suffix");
        if matches!(c, ' ' | '\t' | '\r' | '\n') {
            i += c.len_utf8();
            continue;
        }
        if c == '#' {
            i = source[i..].find('\n').map_or(source.len(), |n| i + n + 1);
            continue;
        }
        let (kind, value) = if c == '"' {
            i += 1;
            let mut value = String::new();
            let mut closed = false;
            while i < source.len() {
                let mut c = source[i..].chars().next().expect("UTF-8 suffix");
                i += c.len_utf8();
                if c == '"' {
                    closed = true;
                    break;
                }
                if c == '\\' {
                    c = source[i..]
                        .chars()
                        .next()
                        .ok_or_else(|| error("escape", start..i))?;
                    if !matches!(c, '"' | '\\') {
                        return Err(error("escape", i..i + c.len_utf8()));
                    }
                    i += c.len_utf8();
                }
                if c.is_control() || matches!(c, '\u{2028}' | '\u{2029}') {
                    return Err(error("string-control", start..i));
                }
                value.push(c);
            }
            if !closed {
                return Err(error("string-end", start..i));
            }
            (Kind::Text, value)
        } else if "[](),.;".contains(c) {
            i += 1;
            if matches!(c, '[' | '(') {
                delimiters.push(c);
                if delimiters.len() > MAX_DEPTH {
                    return Err(error("depth", start..i));
                }
            } else if matches!(c, ']' | ')')
                && delimiters.pop() != Some(if c == ']' { '[' } else { '(' })
            {
                return Err(error("delimiter", start..i));
            }
            (Kind::Punctuation, c.to_string())
        } else if c == '-' || c.is_ascii_digit() {
            if c == '-' {
                i += 1;
            }
            if !source.as_bytes().get(i).is_some_and(u8::is_ascii_digit) {
                return Err(error("number", start..i));
            }
            if source.as_bytes()[i] == b'0' {
                i += 1;
            } else {
                while source.as_bytes().get(i).is_some_and(u8::is_ascii_digit) {
                    i += 1;
                }
            }
            if source.as_bytes().get(i) == Some(&b'.')
                && source.as_bytes().get(i + 1).is_some_and(u8::is_ascii_digit)
            {
                i += 1;
                while source.as_bytes().get(i).is_some_and(u8::is_ascii_digit) {
                    i += 1;
                }
            }
            if source[i..]
                .chars()
                .next()
                .is_some_and(|c| c.is_alphanumeric() || matches!(c, '_' | '-'))
            {
                return Err(error("number-boundary", start..i));
            }
            (Kind::Number, source[start..i].into())
        } else if c.is_ascii_alphabetic() {
            while source
                .as_bytes()
                .get(i)
                .is_some_and(u8::is_ascii_alphabetic)
            {
                i += 1;
            }
            while source.as_bytes().get(i) == Some(&b'-')
                && source
                    .as_bytes()
                    .get(i + 1)
                    .is_some_and(u8::is_ascii_alphabetic)
            {
                i += 1;
                while source
                    .as_bytes()
                    .get(i)
                    .is_some_and(u8::is_ascii_alphabetic)
                {
                    i += 1;
                }
            }
            (Kind::Word, source[start..i].into())
        } else {
            return Err(error("token", start..start + c.len_utf8()));
        };
        tokens.push(Token {
            kind,
            value,
            span: start..i,
        });
        if tokens.len() > MAX_TOKENS {
            return Err(error("tokens", start..i));
        }
    }
    if !delimiters.is_empty() {
        return Err(error("delimiter", source.len()..source.len()));
    }
    Ok(tokens)
}

#[derive(Debug)]
enum Pattern {
    Terminal(&'static str),
    Reference(&'static str),
    Sequence(Vec<Pattern>),
    Choice(Vec<Pattern>),
    Optional(Box<Pattern>),
    Repeat(Box<Pattern>, bool),
}

struct Meta {
    text: &'static str,
    position: usize,
}
impl Meta {
    fn whitespace(&mut self) {
        while self
            .text
            .as_bytes()
            .get(self.position)
            .is_some_and(u8::is_ascii_whitespace)
        {
            self.position += 1;
        }
    }
    fn take(&mut self, c: u8) -> bool {
        self.whitespace();
        if self.text.as_bytes().get(self.position) == Some(&c) {
            self.position += 1;
            true
        } else {
            false
        }
    }
    fn choice(&mut self) -> Pattern {
        let mut items = vec![self.sequence()];
        while self.take(b'|') {
            items.push(self.sequence());
        }
        Pattern::Choice(items)
    }
    fn sequence(&mut self) -> Pattern {
        let mut items = Vec::new();
        loop {
            self.whitespace();
            let Some(&c) = self.text.as_bytes().get(self.position) else {
                break;
            };
            if matches!(c, b'|' | b')' | b';') {
                break;
            }
            let mut atom = if self.take(b'(') {
                let p = self.choice();
                assert!(self.take(b')'), "trusted EBNF closing group");
                p
            } else if self.take(b'"') {
                let start = self.position;
                self.position += self.text[start..].find('"').expect("trusted EBNF terminal");
                let word = &self.text[start..self.position];
                self.position += 1;
                Pattern::Terminal(word)
            } else {
                let start = self.position;
                while self
                    .text
                    .as_bytes()
                    .get(self.position)
                    .is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_')
                {
                    self.position += 1;
                }
                assert!(start != self.position, "trusted EBNF reference");
                Pattern::Reference(&self.text[start..self.position])
            };
            if self.take(b'?') {
                atom = Pattern::Optional(Box::new(atom));
            } else if self.take(b'*') {
                atom = Pattern::Repeat(Box::new(atom), false);
            } else if self.take(b'+') {
                atom = Pattern::Repeat(Box::new(atom), true);
            }
            items.push(atom);
        }
        assert!(!items.is_empty(), "trusted EBNF nonempty sequence");
        Pattern::Sequence(items)
    }
}

type Grammar = BTreeMap<&'static str, Pattern>;
fn grammar() -> &'static Grammar {
    static COMPILED: OnceLock<Grammar> = OnceLock::new();
    COMPILED.get_or_init(|| {
        GRAMMAR
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .map(|line| {
                let (name, body) = line.split_once('=').expect("trusted EBNF rule");
                let mut parser = Meta {
                    text: body,
                    position: 0,
                };
                let rule = parser.choice();
                assert!(
                    parser.take(b';') && parser.position == body.len(),
                    "trusted EBNF end"
                );
                (name.trim(), rule)
            })
            .collect()
    })
}

struct Recognizer<'a> {
    tokens: &'a [Token],
    position: usize,
    work: usize,
    furthest: usize,
}
impl Recognizer<'_> {
    fn rule(&mut self, name: &'static str, depth: usize) -> Result<Option<Tree>> {
        if let Some(pattern) = grammar().get(name) {
            let start = self.position;
            Ok(self.pattern(pattern, depth)?.map(|children| Tree {
                rule: name,
                tokens: start..self.position,
                children,
            }))
        } else {
            let token = self.tokens.get(self.position);
            let matches = token.is_some_and(|t| match name {
                "TEXT" => t.kind == Kind::Text,
                "NAME" => t.kind == Kind::Text && !t.value.is_empty(),
                "NUMBER" => t.kind == Kind::Number,
                "INTEGER" => t.kind == Kind::Number && !t.value.contains('.') && t.value != "-0",
                "NAT" => {
                    t.kind == Kind::Number && !t.value.contains('.') && !t.value.starts_with('-')
                }
                _ => false,
            });
            Ok(if matches {
                let start = self.position;
                self.position += 1;
                Some(Tree {
                    rule: name,
                    tokens: start..self.position,
                    children: vec![],
                })
            } else {
                self.furthest = self.furthest.max(self.position);
                None
            })
        }
    }
    fn pattern(&mut self, p: &Pattern, depth: usize) -> Result<Option<Vec<Tree>>> {
        self.work += 1;
        if self.work > MAX_WORK || depth > 512 {
            return Err(error(
                "parse-budget",
                self.tokens
                    .get(self.position)
                    .map_or(0..0, |t| t.span.clone()),
            ));
        }
        let start = self.position;
        let result = match p {
            Pattern::Terminal(word) => {
                if self.tokens.get(self.position).is_some_and(|t| {
                    matches!(t.kind, Kind::Word | Kind::Punctuation) && t.value == *word
                }) {
                    self.position += 1;
                    Some(vec![])
                } else {
                    self.furthest = self.furthest.max(self.position);
                    None
                }
            }
            Pattern::Reference(name) => self.rule(name, depth + 1)?.map(|node| vec![node]),
            Pattern::Sequence(items) => {
                let mut nodes = Vec::new();
                let mut success = true;
                for item in items {
                    if let Some(children) = self.pattern(item, depth + 1)? {
                        nodes.extend(children);
                    } else {
                        success = false;
                        break;
                    }
                }
                success.then_some(nodes)
            }
            Pattern::Choice(items) => {
                let mut result = None;
                for item in items {
                    self.position = start;
                    if let Some(nodes) = self.pattern(item, depth + 1)? {
                        result = Some(nodes);
                        break;
                    }
                }
                result
            }
            Pattern::Optional(item) => Some(self.pattern(item, depth + 1)?.unwrap_or_default()),
            Pattern::Repeat(item, required) => {
                let mut nodes = Vec::new();
                let mut count = 0;
                loop {
                    let before = self.position;
                    let Some(children) = self.pattern(item, depth + 1)? else {
                        break;
                    };
                    if self.position == before {
                        return Err(error("grammar-repeat", 0..0));
                    }
                    nodes.extend(children);
                    count += 1;
                }
                (count > 0 || !required).then_some(nodes)
            }
        };
        if result.is_none() {
            self.position = start;
        }
        Ok(result)
    }
}

/// Parse a full profile 0.2.0 source, preserving exact source and UTF-8 spans.
pub fn parse(raw: &[u8]) -> Result<Syntax<'_>> {
    parse_fragment(raw, "program")
}

/// Syntax-test entry points only. Fragments lack binding/semantic authority.
pub fn parse_fragment<'a>(raw: &'a [u8], rule: &str) -> Result<Syntax<'a>> {
    if raw.len() > MAX_BYTES {
        return Err(error("size", 0..raw.len()));
    }
    let source = std::str::from_utf8(raw).map_err(|e| {
        error(
            "utf8",
            e.valid_up_to()..e.valid_up_to() + e.error_len().unwrap_or(0),
        )
    })?;
    if source.starts_with('\u{feff}') {
        return Err(error("bom", 0..3));
    }
    let tokens = lex(source)?;
    if rule == "program"
        && !tokens
            .get(2)
            .is_some_and(|t| t.kind == Kind::Text && t.value == "0.2.0")
    {
        return Err(error("profile", 0..source.len()));
    }
    let (&name, _) = grammar()
        .get_key_value(rule)
        .ok_or_else(|| error("grammar-rule", 0..0))?;
    let mut recognizer = Recognizer {
        tokens: &tokens,
        position: 0,
        work: 0,
        furthest: 0,
    };
    let tree = recognizer
        .rule(name, 0)?
        .filter(|_| recognizer.position == tokens.len())
        .ok_or_else(|| {
            error(
                "syntax",
                tokens
                    .get(recognizer.furthest.max(recognizer.position))
                    .map_or(source.len()..source.len(), |t| t.span.clone()),
            )
        })?;
    Ok(Syntax {
        source,
        tokens,
        tree,
    })
}
