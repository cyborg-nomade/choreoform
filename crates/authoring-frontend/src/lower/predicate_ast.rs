// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Bounded private expression assembly/tracing, stripped before wire IR.
use super::*;
pub(super) const MARKER: &str = "__source_expect";
pub(super) const BOOLEAN: &str = "@predicate:boolean";
#[derive(Clone, Copy, Debug)]
pub(super) struct Cost {
    pub work: usize,
    pub bytes: usize,
    pub depth: usize,
}
impl Cost {
    fn checked(self) -> Result<Self> {
        if self.work > crate::MAX_WORK || self.bytes > crate::MAX_BYTES {
            return Err(error("predicate-expansion", 0..0));
        }
        if self.depth > MAX_DEPTH {
            return Err(error("depth", 0..0));
        }
        Ok(self)
    }
    pub fn raw(value: &Value, depth: usize) -> Result<Self> {
        if depth > MAX_DEPTH {
            return Err(error("depth", 0..0));
        }
        match value {
            Value::Object(map) => {
                let mut result = Self {
                    work: 1,
                    bytes: 2,
                    depth: 1,
                };
                for (index, (name, value)) in map.iter().enumerate() {
                    let child = Self::raw(value, depth + 1)?;
                    let field = Self::object([(name.as_str(), child)])?;
                    result.work = result
                        .work
                        .checked_add(field.work - 1)
                        .ok_or_else(|| error("predicate-expansion", 0..0))?;
                    result.bytes = result
                        .bytes
                        .checked_add(field.bytes - 2 + usize::from(index != 0))
                        .ok_or_else(|| error("predicate-expansion", 0..0))?;
                    result.depth = result.depth.max(field.depth);
                    result.checked()?;
                }
                result.checked()
            }
            Value::Array(list) => {
                let mut result = Self {
                    work: 1,
                    bytes: 2,
                    depth: 1,
                };
                for (index, value) in list.iter().enumerate() {
                    let child = Self::raw(value, depth + 1)?;
                    result.work = result
                        .work
                        .checked_add(child.work)
                        .ok_or_else(|| error("predicate-expansion", 0..0))?;
                    result.bytes = result
                        .bytes
                        .checked_add(child.bytes + usize::from(index != 0))
                        .ok_or_else(|| error("predicate-expansion", 0..0))?;
                    result.depth = result.depth.max(1 + child.depth);
                    result.checked()?;
                }
                result.checked()
            }
            _ => Self {
                work: 1,
                bytes: protection::encoded_size(value)?,
                depth: 0,
            }
            .checked(),
        }
    }
    fn object<'b>(fields: impl IntoIterator<Item = (&'b str, Self)>) -> Result<Self> {
        let mut c = Self {
            work: 1,
            bytes: 2,
            depth: 1,
        };
        let mut first = true;
        for (name, child) in fields {
            let key = protection::encoded_size(&Value::String(name.into()))?;
            c.work = c
                .work
                .checked_add(1 + child.work)
                .ok_or_else(|| error("predicate-expansion", 0..0))?;
            c.bytes = c
                .bytes
                .checked_add(key + 1 + child.bytes + usize::from(!first))
                .ok_or_else(|| error("predicate-expansion", 0..0))?;
            c.depth = c.depth.max(1 + child.depth);
            first = false;
            c.checked()?;
        }
        c.checked()
    }
    fn array(items: impl IntoIterator<Item = Self>) -> Result<Self> {
        let mut c = Self {
            work: 1,
            bytes: 2,
            depth: 1,
        };
        let mut first = true;
        for child in items {
            c.work = c
                .work
                .checked_add(child.work)
                .ok_or_else(|| error("predicate-expansion", 0..0))?;
            c.bytes = c
                .bytes
                .checked_add(child.bytes + usize::from(!first))
                .ok_or_else(|| error("predicate-expansion", 0..0))?;
            c.depth = c.depth.max(1 + child.depth);
            first = false;
            c.checked()?;
        }
        c.checked()
    }
}
pub(super) struct Expr {
    pub value: Value,
    pub cost: Cost,
}
impl Expr {
    pub fn raw(value: Value) -> Result<Self> {
        let cost = Cost::raw(&value, 0)?;
        Ok(Self { value, cost })
    }
    pub fn object(fields: Vec<(&str, Self)>) -> Result<Self> {
        let cost = Cost::object(fields.iter().map(|(k, v)| (*k, v.cost)))?;
        let mut map = Map::new();
        for (key, value) in fields {
            insert(&mut map, key.into(), value.value)?;
        }
        Ok(Self {
            value: Value::Object(map),
            cost,
        })
    }
    pub fn named_object(fields: Vec<(String, Self)>) -> Result<Self> {
        let cost = Cost::object(fields.iter().map(|(k, v)| (k.as_str(), v.cost)))?;
        let mut map = Map::new();
        for (key, value) in fields {
            insert(&mut map, key, value.value)?;
        }
        Ok(Self {
            value: Value::Object(map),
            cost,
        })
    }
    pub fn array(items: Vec<Self>) -> Result<Self> {
        let cost = Cost::array(items.iter().map(|v| v.cost))?;
        Ok(Self {
            value: Value::Array(items.into_iter().map(|v| v.value).collect()),
            cost,
        })
    }
}
pub(super) fn scalar(value: impl Into<Value>) -> Result<Expr> {
    Expr::raw(value.into())
}
pub(super) fn node(op: &str, mut fields: Vec<(&str, Expr)>) -> Result<Expr> {
    fields.push(("op", scalar(op)?));
    Expr::object(fields)
}
pub(super) fn expect(value: Expr, key: &str, span: std::ops::Range<usize>) -> Result<Expr> {
    node(
        MARKER,
        vec![
            ("value", value),
            ("expected", scalar(key)?),
            ("start", scalar(span.start)?),
            ("end", scalar(span.end)?),
        ],
    )
}
#[derive(Clone)]
pub(super) struct Assertion {
    pub path: String,
    pub key: String,
    pub span: std::ops::Range<usize>,
}
struct Trace<'a> {
    out: &'a mut Vec<Assertion>,
    bytes: usize,
    limit: usize,
    work: usize,
}
/// Reuse one path buffer; materialize paths only for bounded assertion rows.
/// This avoids copying a long common field prefix at every ordinary AST node.
pub(super) fn strip(
    value: &mut Value,
    path: &str,
    out: &mut Vec<Assertion>,
    depth: usize,
    limit: usize,
) -> Result<usize> {
    let mut trace = Trace {
        out,
        bytes: 0,
        limit,
        work: 0,
    };
    strip_inner(value, &mut path.to_owned(), &mut trace, depth)?;
    Ok(trace.bytes)
}
fn descend(
    value: &mut Value,
    path: &mut String,
    key: &str,
    trace: &mut Trace<'_>,
    depth: usize,
) -> Result<()> {
    let old = path.len();
    let size = key
        .bytes()
        .try_fold(old + 1, |n, b| {
            n.checked_add(if b == b'~' || b == b'/' { 2 } else { 1 })
        })
        .filter(|n| *n <= crate::MAX_BYTES)
        .ok_or_else(|| error("predicate-expansion", 0..0))?;
    path.reserve(size - old);
    path.push('/');
    for c in key.chars() {
        match c {
            '~' => path.push_str("~0"),
            '/' => path.push_str("~1"),
            _ => path.push(c),
        }
    }
    let result = strip_inner(value, path, trace, depth + 1);
    path.truncate(old);
    result
}
fn strip_inner(
    value: &mut Value,
    path: &mut String,
    trace: &mut Trace<'_>,
    depth: usize,
) -> Result<()> {
    trace.work = trace
        .work
        .checked_add(1)
        .filter(|n| *n <= crate::MAX_WORK)
        .ok_or_else(|| error("predicate-expansion", 0..0))?;
    if depth > MAX_DEPTH {
        return Err(error("depth", 0..0));
    }
    while value["op"] == MARKER {
        trace.work = trace
            .work
            .checked_add(1)
            .filter(|n| *n <= crate::MAX_WORK)
            .ok_or_else(|| error("predicate-expansion", 0..0))?;
        let key = value["expected"]
            .as_str()
            .ok_or_else(|| error("predicate-metadata", 0..0))?;
        let size = path
            .len()
            .checked_add(key.len())
            .and_then(|n| n.checked_add(24))
            .ok_or_else(|| error("predicate-expansion", 0..0))?;
        trace.bytes = trace
            .bytes
            .checked_add(size)
            .filter(|n| *n <= trace.limit)
            .ok_or_else(|| error("predicate-expansion", 0..0))?;
        if trace.out.len() >= 10_000 {
            return Err(error("predicate-expansion", 0..0));
        }
        let start = value["start"]
            .as_u64()
            .and_then(|v| usize::try_from(v).ok())
            .ok_or_else(|| error("predicate-metadata", 0..0))?;
        let end = value["end"]
            .as_u64()
            .and_then(|v| usize::try_from(v).ok())
            .ok_or_else(|| error("predicate-metadata", 0..0))?;
        if start > end || end > crate::MAX_BYTES {
            return Err(error("predicate-metadata", 0..0));
        }
        trace.out.push(Assertion {
            path: path.clone(),
            key: key.into(),
            span: start..end,
        });
        *value = value["value"].take();
    }
    if value["op"] == "literal" {
        return Ok(());
    }
    let expression = value.get("op").is_some_and(Value::is_string);
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                if !expression || (key != "type" && key != "element") {
                    descend(child, path, key, trace, depth)?;
                }
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter_mut().enumerate() {
                descend(child, path, &index.to_string(), trace, depth)?;
            }
        }
        _ => {}
    }
    Ok(())
}

pub(super) struct Argument<'a> {
    pub expr: &'a Expr,
    pub key: &'a str,
    pub span: std::ops::Range<usize>,
}
fn parameter<'a, 'b>(
    value: &Value,
    args: &'a BTreeMap<String, Argument<'b>>,
) -> Result<Option<(&'a str, &'a Argument<'b>)>> {
    if value["op"] != "param" {
        return Ok(None);
    }
    let wire = value["name"]
        .as_str()
        .ok_or_else(|| error("predicate-parameter", 0..0))?;
    args.get_key_value(wire)
        .map(|(k, v)| Some((k.as_str(), v)))
        .ok_or_else(|| error("predicate-parameter", 0..0))
}
/// Predict the complete substituted allocation before copying any cached body or
/// duplicated argument. Literal data/types are opaque to binding.
pub(super) fn substitution_cost(
    value: &Value,
    args: &BTreeMap<String, Argument<'_>>,
    seen: &mut BTreeSet<String>,
    call: &std::ops::Range<usize>,
    depth: usize,
) -> Result<Cost> {
    if depth > MAX_DEPTH {
        return Err(error("depth", 0..0));
    }
    if let Some((wire, arg)) = parameter(value, args)? {
        return if seen.insert(wire.into()) {
            expect_cost(arg.expr.cost, arg.key, &arg.span)
        } else {
            Ok(arg.expr.cost)
        };
    }
    if value["op"] == "literal" {
        return Cost::raw(value, depth);
    }
    let expression = value.get("op").is_some_and(Value::is_string);
    match value {
        Value::Object(map) => {
            let mut fields = Vec::new();
            for (key, child) in map {
                let cost = if value["op"] == MARKER && (key == "start" || key == "end") {
                    Cost::raw(
                        &Value::from(if key == "start" { call.start } else { call.end }),
                        0,
                    )?
                } else if expression && (key == "type" || key == "element") {
                    Cost::raw(child, depth + 1)?
                } else {
                    substitution_cost(child, args, seen, call, depth + 1)?
                };
                fields.push((key.as_str(), cost));
            }
            Cost::object(fields)
        }
        Value::Array(list) => Cost::array(
            list.iter()
                .map(|v| substitution_cost(v, args, seen, call, depth + 1))
                .collect::<Result<Vec<_>>>()?,
        ),
        _ => Cost::raw(value, depth),
    }
}
fn expect_cost(value: Cost, key: &str, span: &std::ops::Range<usize>) -> Result<Cost> {
    Cost::object([
        ("op", Cost::raw(&Value::from(MARKER), 0)?),
        ("value", value),
        ("expected", Cost::raw(&Value::from(key), 0)?),
        ("start", Cost::raw(&Value::from(span.start), 0)?),
        ("end", Cost::raw(&Value::from(span.end), 0)?),
    ])
}
pub(super) fn result_cost(value: Cost, span: &std::ops::Range<usize>) -> Result<Cost> {
    expect_cost(value, BOOLEAN, span)
}
pub(super) fn substitute(
    value: &Value,
    args: &BTreeMap<String, Argument<'_>>,
    seen: &mut BTreeSet<String>,
    call: &std::ops::Range<usize>,
    depth: usize,
) -> Result<Expr> {
    if depth > MAX_DEPTH {
        return Err(error("depth", 0..0));
    }
    if let Some((wire, arg)) = parameter(value, args)? {
        let copy = Expr {
            value: arg.expr.value.clone(),
            cost: arg.expr.cost,
        };
        return if seen.insert(wire.into()) {
            expect(copy, arg.key, arg.span.clone())
        } else {
            Ok(copy)
        };
    }
    if value["op"] == "literal" {
        return Expr::raw(value.clone());
    }
    let expression = value.get("op").is_some_and(Value::is_string);
    match value {
        Value::Object(map) => {
            let mut fields = Vec::new();
            for (key, child) in map {
                let child = if value["op"] == MARKER && (key == "start" || key == "end") {
                    scalar(if key == "start" { call.start } else { call.end })?
                } else if expression && (key == "type" || key == "element") {
                    Expr::raw(child.clone())?
                } else {
                    substitute(child, args, seen, call, depth + 1)?
                };
                fields.push((key.clone(), child));
            }
            Expr::named_object(fields)
        }
        Value::Array(list) => Expr::array(
            list.iter()
                .map(|v| substitute(v, args, seen, call, depth + 1))
                .collect::<Result<Vec<_>>>()?,
        ),
        _ => Expr::raw(value.clone()),
    }
}
pub(super) fn parameters(value: &Value, out: &mut BTreeSet<String>, depth: usize) -> Result<()> {
    if depth > MAX_DEPTH {
        return Err(error("depth", 0..0));
    }
    if value["op"] == "param" {
        out.insert(
            value["name"]
                .as_str()
                .ok_or_else(|| error("predicate-parameter", 0..0))?
                .into(),
        );
        return Ok(());
    }
    if value["op"] == "literal" {
        return Ok(());
    }
    let expression = value.get("op").is_some_and(Value::is_string);
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                if !expression || (key != "type" && key != "element") {
                    parameters(child, out, depth + 1)?;
                }
            }
        }
        Value::Array(list) => {
            for child in list {
                parameters(child, out, depth + 1)?;
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn escaped_long_prefixes_count_every_retained_assertion_before_copying() {
        let leaf = || expect(scalar(true).unwrap(), BOOLEAN, 1..2).unwrap().value;
        let key = "a~/".repeat(1000);
        let mut value = json!({key: [leaf(),leaf(),leaf()]});
        let mut rows = Vec::new();
        assert_eq!(
            strip(&mut value, "", &mut rows, 0, 12000).unwrap_err().code,
            "predicate-expansion"
        );
        assert_eq!(rows.len(), 2);
        assert!(rows[0].path.starts_with("/a~0~1"));
        assert!(rows[0].path.ends_with("/0"));
    }
    #[test]
    fn marker_like_literal_data_is_not_source_metadata() {
        let payload = json!({"op":MARKER,"expected":"SECRET","start":0,"end":1,"value":true});
        let mut value = json!({"op":"literal","type":{"kind":"record"},"value":payload});
        let expected = value.clone();
        let mut rows = Vec::new();
        assert_eq!(strip(&mut value, "", &mut rows, 0, 0).unwrap(), 0);
        assert!(rows.is_empty());
        assert_eq!(value, expected);
    }
}
