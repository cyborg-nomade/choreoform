// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Read-only fixture measurements. No new syntax, expansion or usability verdict.
use choreoform_authoring_frontend::{MAX_BYTES, Syntax, Tree, parse};
use choreoform_ir_probe_core::digest;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::Read;

#[derive(Default)]
struct Repetition {
    occurrences: usize,
    bytes: usize,
    spellings: BTreeMap<String, usize>,
}
impl Repetition {
    fn add(&mut self, text: &str) {
        self.occurrences += 1;
        self.bytes += text.len();
        *self.spellings.entry(text.into()).or_default() += 1;
    }
    fn report(self) -> Value {
        let distinct = self.spellings.len();
        let redundant_bytes: usize = self.spellings.iter().map(|(s, n)| s.len() * (n - 1)).sum();
        let mut repeated: Vec<_> = self.spellings.into_iter().filter(|(_, n)| *n > 1).collect();
        repeated.sort_by(|(a, an), (b, bn)| {
            (b.len() * (bn - 1))
                .cmp(&(a.len() * (an - 1)))
                .then_with(|| a.cmp(b))
        });
        json!({"occurrences":self.occurrences,"distinctSpellings":distinct,"bytes":self.bytes,"repeatedBytesUpperBound":redundant_bytes,
               "topRepeated":repeated.into_iter().take(8).map(|(s,n)|json!({"spellingDigest":digest(s.as_bytes()),"bytesEach":s.len(),"occurrences":n})).collect::<Vec<_>>()})
    }
}
fn measure(syntax: &Syntax<'_>) -> Result<Value, &'static str> {
    measure_with_budget(syntax, 1_000_000)
}
fn measure_with_budget(syntax: &Syntax<'_>, limit: usize) -> Result<Value, &'static str> {
    let mut stack: Vec<(&Tree, bool, bool)> = vec![(syntax.tree(), false, false)];
    let mut work = 0usize;
    let mut rules: BTreeMap<&str, usize> = BTreeMap::new();
    let mut types = Repetition::default();
    let mut expressions = Repetition::default();
    let mut protection = Repetition::default();
    let mut qualified_references = 0usize;
    while let Some((tree, in_type, in_expression)) = stack.pop() {
        work += 1;
        if work > limit {
            return Err("measurement work bound");
        }
        *rules.entry(tree.rule).or_default() += 1;
        if tree.rule == "reference" && tree.children.iter().filter(|t| t.rule == "NAME").count() > 1
        {
            qualified_references += 1;
        }
        let is_type = tree.rule == "type";
        let is_expression = tree.rule == "expression";
        if is_type && !in_type {
            types.add(&syntax.source()[syntax.span(tree)]);
        }
        if is_expression && !in_expression {
            expressions.add(&syntax.source()[syntax.span(tree)]);
        }
        if tree.rule == "data" {
            let policy = tree
                .children
                .iter()
                .find(|t| t.rule == "reference")
                .ok_or("measurement data policy shape")?;
            let references: Vec<_> = tree
                .children
                .iter()
                .filter(|t| t.rule == "references")
                .collect();
            if references.len() != 3 {
                return Err("measurement data references shape");
            }
            // Under-policy through capabilities; invalidation and initial value
            // are excluded. Same spelling need not mean the same lexical binding.
            protection
                .add(&syntax.source()[syntax.span(policy).start..syntax.span(references[1]).end]);
        }
        stack.extend(
            tree.children
                .iter()
                .map(|child| (child, in_type || is_type, in_expression || is_expression)),
        );
    }
    Ok(
        json!({"profile":syntax.profile(),"sourceDigest":digest(syntax.source().as_bytes()),"sourceBytes":syntax.source().len(),"tokens":syntax.token_count(),"parserWork":syntax.parser_work(),
              "outerTypes":types.report(),"outerExpressions":expressions.report(),"dataProtection":protection.report(),"qualifiedReferences":qualified_references,"ruleCounts":rules,
              "interpretation":"exact spelling repetition only; categories can overlap; upper bounds exclude alias definition/reference costs; no usability or lowering-equivalence claim"}),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths: Vec<_> = std::env::args().skip(1).collect();
    if paths.is_empty() || paths.len() > 3 {
        return Err("supply one to three bounded fixture paths".into());
    }
    let mut reports = vec![];
    for path in paths {
        if path.len() > 4096 {
            return Err("path bound".into());
        }
        let mut raw = vec![];
        std::fs::File::open(&path)?
            .take((MAX_BYTES + 1) as u64)
            .read_to_end(&mut raw)?;
        let syntax =
            parse(&raw).map_err(|e| format!("{} at {}..{}", e.code, e.span.start, e.span.end))?;
        let mut report = measure(&syntax)?;
        report["fixture"] = path.into();
        reports.push(report);
    }
    let raw = serde_json::to_vec(&reports)?;
    if raw.len() > 65536 {
        return Err("measurement output bound".into());
    }
    println!("{}", String::from_utf8(raw)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use choreoform_authoring_frontend::parse_fragment;
    #[test]
    fn qualification_is_structural_not_an_in_word_inside_a_name() {
        for (source, expected) in [
            ("\"Value\" in \"Review\" in \"Process\"", 1),
            ("\"Value in Review\"", 0),
        ] {
            let syntax = parse_fragment(source.as_bytes(), "reference").unwrap();
            assert_eq!(measure(&syntax).unwrap()["qualifiedReferences"], expected);
        }
    }
    #[test]
    fn exhausted_measurement_returns_no_partial_report() {
        let syntax = parse_fragment(b"optional text", "type").unwrap();
        assert_eq!(
            measure_with_budget(&syntax, 0),
            Err("measurement work bound")
        );
        assert!(measure(&syntax).is_ok());
    }
    #[test]
    fn nested_types_and_expressions_have_only_one_outer_extent() {
        for (rule, source, category) in [
            (
                "type",
                "record with [\"first\" as record with [\"n\" as whole number], \"second\" as optional text]",
                "outerTypes",
            ),
            ("expression", "(true) and (not (false))", "outerExpressions"),
        ] {
            let syntax = parse_fragment(source.as_bytes(), rule).unwrap();
            let report = measure(&syntax).unwrap();
            assert_eq!(report[category]["occurrences"], 1);
            assert_eq!(report[category]["bytes"], source.len());
            assert!(report["ruleCounts"][rule].as_u64().unwrap() > 1);
        }
    }
    #[test]
    fn changing_only_invalidation_does_not_change_measured_protection_clause() {
        let data = |invalidates: &str| {
            format!(
                "Keep \"Value\" as whole number under \"Access\". Sensitivity \"test\". Purposes empty. Participants empty. Capabilities empty. Invalidate {invalidates}. Initially unavailable. End data."
            )
        };
        let first = data("empty");
        let second = data("[\"Step\"]");
        let a = measure(&parse_fragment(first.as_bytes(), "data").unwrap()).unwrap();
        let b = measure(&parse_fragment(second.as_bytes(), "data").unwrap()).unwrap();
        assert_eq!(a["dataProtection"], b["dataProtection"]);
        assert_ne!(a["sourceBytes"], b["sourceBytes"]);
    }
}
