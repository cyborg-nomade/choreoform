// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

use crate::{MAX_BYTES, MAX_DEPTH, MAX_TOKENS, MAX_WORK, Result, error};
use std::{collections::BTreeMap, ops::Range, sync::OnceLock};

const GRAMMAR: &str = include_str!("../../../docs/authoring/candidate-a.ebnf");
const GRAMMAR_V3: &str = include_str!("../../../docs/authoring/candidate-a-v0.3.ebnf");
const GRAMMAR_V7: &str = include_str!("../../../docs/authoring/candidate-a-v0.7.ebnf");
const GRAMMAR_V8: &str = include_str!("../../../docs/authoring/candidate-a-v0.8.ebnf");
const GRAMMAR_V19: &str = include_str!("../../../docs/authoring/candidate-a-v0.19.ebnf");
const GRAMMAR_V18: &str = include_str!("../../../docs/authoring/candidate-a-v0.18.ebnf");
const GRAMMAR_V17: &str = include_str!("../../../docs/authoring/candidate-a-v0.17.ebnf");
const GRAMMAR_V16: &str = include_str!("../../../docs/authoring/candidate-a-v0.16.ebnf");
const GRAMMAR_V15: &str = include_str!("../../../docs/authoring/candidate-a-v0.15.ebnf");
const GRAMMAR_V14: &str = include_str!("../../../docs/authoring/candidate-a-v0.14.ebnf");
const GRAMMAR_V13: &str = include_str!("../../../docs/authoring/candidate-a-v0.13.ebnf");
const GRAMMAR_V12: &str = include_str!("../../../docs/authoring/candidate-a-v0.12.ebnf");
const GRAMMAR_V11: &str = include_str!("../../../docs/authoring/candidate-a-v0.11.ebnf");
const GRAMMAR_V10: &str = include_str!("../../../docs/authoring/candidate-a-v0.10.ebnf");
const GRAMMAR_V9: &str = include_str!("../../../docs/authoring/candidate-a-v0.9.ebnf");
const V5_TOKENS: usize = 16_384;
const V15_TOKENS: usize = 131_072;
const V5_WORK: usize = 1_000_000;

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
    profile: &'static str,
    tokens: Vec<Token>,
    tree: Tree,
    parser_work: usize,
}

impl Syntax<'_> {
    pub fn profile(&self) -> &'static str {
        self.profile
    }
    /// Deterministic lexical size, not semantic complexity or execution evidence.
    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }
    /// Recognizer pattern visits; neither wall time nor a semantic proof.
    pub fn parser_work(&self) -> usize {
        self.parser_work
    }
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

fn lex(source: &str, program: bool) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut delimiters = Vec::new();
    let mut token_limit = MAX_TOKENS;
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
        // Only a complete explicit known header prefix selects the new closed
        // quota. Fragments and arbitrary third strings retain legacy limits.
        if program
            && tokens.len() == 3
            && tokens[0].kind == Kind::Word
            && tokens[0].value == "Choreoform"
            && tokens[1].kind == Kind::Word
            && tokens[1].value == "authoring"
            && tokens[2].kind == Kind::Text
            && matches!(
                tokens[2].value.as_str(),
                "0.5.0"
                    | "0.6.0"
                    | "0.7.0"
                    | "0.8.0"
                    | "0.9.0"
                    | "0.10.0"
                    | "0.11.0"
                    | "0.12.0"
                    | "0.13.0"
                    | "0.14.0"
                    | "0.15.0"
                    | "0.16.0"
                    | "0.17.0"
                    | "0.18.0"
                    | "0.19.0"
            )
        {
            token_limit = if matches!(
                tokens[2].value.as_str(),
                "0.15.0" | "0.16.0" | "0.17.0" | "0.18.0" | "0.19.0"
            ) {
                V15_TOKENS
            } else {
                V5_TOKENS
            };
        }
        if tokens.len() > token_limit {
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
fn grammar(profile: &str) -> &'static Grammar {
    static COMPILED: OnceLock<Grammar> = OnceLock::new();
    static COMPILED_V3: OnceLock<Grammar> = OnceLock::new();
    static COMPILED_V7: OnceLock<Grammar> = OnceLock::new();
    static COMPILED_V8: OnceLock<Grammar> = OnceLock::new();
    static COMPILED_V9: OnceLock<Grammar> = OnceLock::new();
    static COMPILED_V10: OnceLock<Grammar> = OnceLock::new();
    static COMPILED_V11: OnceLock<Grammar> = OnceLock::new();
    static COMPILED_V12: OnceLock<Grammar> = OnceLock::new();
    static COMPILED_V13: OnceLock<Grammar> = OnceLock::new();
    static COMPILED_V14: OnceLock<Grammar> = OnceLock::new();
    static COMPILED_V15: OnceLock<Grammar> = OnceLock::new();
    static COMPILED_V19: OnceLock<Grammar> = OnceLock::new();
    static COMPILED_V18: OnceLock<Grammar> = OnceLock::new();
    static COMPILED_V17: OnceLock<Grammar> = OnceLock::new();
    static COMPILED_V16: OnceLock<Grammar> = OnceLock::new();
    let (cache, source) = if profile == "0.19.0" {
        (&COMPILED_V19, GRAMMAR_V19)
    } else if profile == "0.18.0" {
        (&COMPILED_V18, GRAMMAR_V18)
    } else if profile == "0.17.0" {
        (&COMPILED_V17, GRAMMAR_V17)
    } else if profile == "0.16.0" {
        (&COMPILED_V16, GRAMMAR_V16)
    } else if profile == "0.15.0" {
        (&COMPILED_V15, GRAMMAR_V15)
    } else if profile == "0.14.0" {
        (&COMPILED_V14, GRAMMAR_V14)
    } else if profile == "0.13.0" {
        (&COMPILED_V13, GRAMMAR_V13)
    } else if profile == "0.12.0" {
        (&COMPILED_V12, GRAMMAR_V12)
    } else if profile == "0.11.0" {
        (&COMPILED_V11, GRAMMAR_V11)
    } else if profile == "0.10.0" {
        (&COMPILED_V10, GRAMMAR_V10)
    } else if profile == "0.9.0" {
        (&COMPILED_V9, GRAMMAR_V9)
    } else if profile == "0.8.0" {
        (&COMPILED_V8, GRAMMAR_V8)
    } else if profile == "0.7.0" {
        (&COMPILED_V7, GRAMMAR_V7)
    } else if profile != "0.2.0" {
        (&COMPILED_V3, GRAMMAR_V3)
    } else {
        (&COMPILED, GRAMMAR)
    };
    cache.get_or_init(|| {
        source
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
    grammar: &'static Grammar,
    tokens: &'a [Token],
    position: usize,
    work: usize,
    furthest: usize,
    work_limit: usize,
}
impl Recognizer<'_> {
    fn rule(&mut self, name: &'static str, depth: usize) -> Result<Option<Tree>> {
        if let Some(pattern) = self.grammar.get(name) {
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
        if self.work > self.work_limit || depth > 512 {
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

/// Parse a supported explicitly versioned source, retaining UTF-8 spans.
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
    let tokens = lex(source, rule == "program")?;
    let profile = if rule == "program" {
        match tokens
            .get(2)
            .filter(|t| t.kind == Kind::Text)
            .map(|t| t.value.as_str())
        {
            Some("0.2.0") => "0.2.0",
            Some("0.3.0") => "0.3.0",
            Some("0.4.0") => "0.4.0",
            Some("0.5.0") => "0.5.0",
            Some("0.6.0") => "0.6.0",
            Some("0.7.0") => "0.7.0",
            Some("0.8.0") => "0.8.0",
            Some("0.9.0") => "0.9.0",
            Some("0.10.0") => "0.10.0",
            Some("0.11.0") => "0.11.0",
            Some("0.12.0") => "0.12.0",
            Some("0.13.0") => "0.13.0",
            Some("0.14.0") => "0.14.0",
            Some("0.15.0") => "0.15.0",
            Some("0.16.0") => "0.16.0",
            Some("0.17.0") => "0.17.0",
            Some("0.18.0") => "0.18.0",
            Some("0.19.0") => "0.19.0",
            _ => return Err(error("profile", 0..source.len())),
        }
    } else {
        "0.2.0"
    };
    let grammar = grammar(profile);
    let (&name, _) = grammar
        .get_key_value(rule)
        .ok_or_else(|| error("grammar-rule", 0..0))?;
    let mut recognizer = Recognizer {
        grammar,
        tokens: &tokens,
        position: 0,
        work: 0,
        furthest: 0,
        work_limit: if matches!(
            profile,
            "0.5.0"
                | "0.6.0"
                | "0.7.0"
                | "0.8.0"
                | "0.9.0"
                | "0.10.0"
                | "0.11.0"
                | "0.12.0"
                | "0.13.0"
                | "0.14.0"
                | "0.15.0"
                | "0.16.0"
                | "0.17.0"
                | "0.18.0"
                | "0.19.0"
        ) {
            V5_WORK
        } else {
            MAX_WORK
        },
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
    let parser_work = recognizer.work;
    Ok(Syntax {
        source,
        profile,
        tokens,
        tree,
        parser_work,
    })
}

#[cfg(test)]
mod capacity_tests {
    use super::*;
    #[test]
    fn exact_token_limits_are_explicit_per_program_profile_and_fragments_stay_legacy() {
        for (profile, limit) in [
            ("0.4.0", MAX_TOKENS),
            ("0.14.0", V5_TOKENS),
            ("0.15.0", V15_TOKENS),
            ("0.16.0", V15_TOKENS),
            ("0.17.0", V15_TOKENS),
            ("0.18.0", V15_TOKENS),
        ] {
            let source = format!(
                "Choreoform authoring \"{profile}\". {}",
                "x ".repeat(limit - 4)
            );
            assert_eq!(lex(&source, true).unwrap().len(), limit);
            assert_eq!(lex(&(source + "x"), true).unwrap_err().code, "tokens");
        }
        let fragment = "x ".repeat(MAX_TOKENS);
        assert_eq!(lex(&fragment, false).unwrap().len(), MAX_TOKENS);
        assert_eq!(lex(&(fragment + "x"), false).unwrap_err().code, "tokens");
    }
}
