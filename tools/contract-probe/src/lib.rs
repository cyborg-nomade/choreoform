// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0

//! Disposable ADR-0013 evidence for selected pure judgments, NOT admission.
//! No wire parser, registry, authentication, type universe, IO or process engine.
//! Inputs are typed synthetic contexts; booleans do not authenticate real facts.
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Encoding,
    Range,
    Scale,
    Inexact,
    Type,
    Unavailable,
    Dependencies,
    Budget,
    InvalidContext,
}

/// V1: exact canonical signed-64 string, never through a floating-point value.
pub fn integer(text: &str) -> Result<i64, Error> {
    let digits = text.strip_prefix('-').unwrap_or(text);
    if digits.is_empty()
        || !digits.bytes().all(|b| b.is_ascii_digit())
        || (digits.starts_with('0') && text != "0")
    {
        return Err(Error::Encoding);
    }
    text.parse().map_err(|_| Error::Range)
}

/// V2: scale changes are exact; no rounding, widening, saturation or wrapping.
pub fn rescale(coefficient: i64, from: u8, to: u8) -> Result<i64, Error> {
    if from > 18 || to > 18 {
        return Err(Error::Scale);
    }
    let factor = 10_i64.pow(u32::from(from.abs_diff(to)));
    if to >= from {
        coefficient.checked_mul(factor).ok_or(Error::Range)
    } else if coefficient % factor == 0 {
        Ok(coefficient / factor)
    } else {
        Err(Error::Inexact)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Type {
    Boolean,
    Integer,
}

impl Value {
    fn ty(self) -> Type {
        match self {
            Self::Boolean(_) => Type::Boolean,
            Self::Integer(_) => Type::Integer,
        }
    }
}

/// Tiny AST subset for dependency and snapshot evidence, not the whole dialect.
#[derive(Debug)]
pub enum Expr {
    Literal(Value),
    Read(String),
    Add(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl Expr {
    fn check(
        &self,
        snapshot: &BTreeMap<String, Value>,
        reads: &mut BTreeSet<String>,
        depth: usize,
        remaining: &mut usize,
    ) -> Result<Type, Error> {
        if depth > 64 || *remaining == 0 {
            return Err(Error::Budget);
        }
        *remaining -= 1;
        match self {
            Self::Literal(v) => Ok(v.ty()),
            Self::Read(id) => {
                reads.insert(id.clone());
                snapshot.get(id).map(|v| v.ty()).ok_or(Error::Unavailable)
            }
            Self::Add(a, b) => {
                let a = a.check(snapshot, reads, depth + 1, remaining)?;
                let b = b.check(snapshot, reads, depth + 1, remaining)?;
                if a != Type::Integer || b != Type::Integer {
                    return Err(Error::Type);
                }
                Ok(Type::Integer)
            }
            Self::If(c, a, b) => {
                let c = c.check(snapshot, reads, depth + 1, remaining)?;
                let a = a.check(snapshot, reads, depth + 1, remaining)?;
                let b = b.check(snapshot, reads, depth + 1, remaining)?;
                if c != Type::Boolean || a != b {
                    return Err(Error::Type);
                }
                Ok(a)
            }
        }
    }

    fn run(&self, snapshot: &BTreeMap<String, Value>) -> Result<Value, Error> {
        match self {
            Self::Literal(v) => Ok(*v),
            Self::Read(id) => snapshot.get(id).copied().ok_or(Error::Unavailable),
            Self::Add(a, b) => match (a.run(snapshot)?, b.run(snapshot)?) {
                (Value::Integer(a), Value::Integer(b)) => {
                    a.checked_add(b).map(Value::Integer).ok_or(Error::Range)
                }
                _ => Err(Error::Type),
            },
            Self::If(c, a, b) => match c.run(snapshot)? {
                Value::Boolean(true) => a.run(snapshot),
                Value::Boolean(false) => b.run(snapshot),
                _ => Err(Error::Type),
            },
        }
    }
}

/// E2 subset: check both branches and complete reads before short-circuiting.
/// Snapshot values stand in for previously admitted cell declarations/revisions.
pub fn evaluate(
    expression: &Expr,
    snapshot: &BTreeMap<String, Value>,
    declared_reads: &BTreeSet<String>,
) -> Result<Value, Error> {
    let mut inferred = BTreeSet::new();
    expression.check(snapshot, &mut inferred, 0, &mut 10_000)?;
    if &inferred != declared_reads {
        return Err(Error::Dependencies);
    }
    expression.run(snapshot)
}

/// P1 subset: synthetic acceptance facts; the host must authenticate these.
pub struct Completion {
    pub assigned: bool,
    pub accepted: bool,
    pub terminal: bool,
    pub same_principal: bool,
    pub same_input_revision: bool,
    pub authority_now: bool,
    pub duplicate: bool,
}

/// Rechecking authority and correlation is mandatory even after assignment.
pub fn accept_completion(c: &Completion) -> bool {
    c.assigned
        && c.accepted
        && !c.terminal
        && c.same_principal
        && c.same_input_revision
        && c.authority_now
        && !c.duplicate
}

/// P4 eligibility after an ended attempt, not authorization to dispatch IO.
pub struct Retry {
    pub attempts: u16,
    pub max_attempts: u16,
    pub eligible_fault: bool,
    pub delay_elapsed: bool,
    pub authority_now: bool,
    pub cancelled: bool,
    pub succeeded: bool,
    pub unresolved: bool,
    pub stable_key_supported: bool,
    pub same_request: bool,
    pub confirmed_no_effect: bool,
}

pub fn may_retry(r: &Retry) -> Result<bool, Error> {
    if r.attempts == 0 || r.max_attempts == 0 || r.max_attempts > 1000 {
        return Err(Error::InvalidContext);
    }
    Ok(r.attempts < r.max_attempts
        && r.eligible_fault
        && r.delay_elapsed
        && r.authority_now
        && !r.cancelled
        && !r.succeeded
        && r.same_request
        && (r.confirmed_no_effect || (r.unresolved && r.stable_key_supported)))
}

/// P5 subset: one clock domain, matching timer revision and complete pauses.
pub fn timer_eligible(
    now: i64,
    due: i64,
    pause_durations: &[i64],
    paused: bool,
    same_revision: bool,
    intervals: Option<&[(i64, i64)]>,
) -> Result<bool, Error> {
    if pause_durations.len() > 10_000 || intervals.is_some_and(|v| v.len() > 10_000) {
        return Err(Error::Budget);
    }
    let mut effective = due;
    for duration in pause_durations {
        if *duration < 0 {
            return Err(Error::InvalidContext);
        }
        effective = effective.checked_add(*duration).ok_or(Error::Range)?;
    }
    if let Some(intervals) = intervals {
        let mut previous_end = None;
        for &(start, end) in intervals {
            if start >= end || previous_end.is_some_and(|p| start < p) {
                return Err(Error::InvalidContext);
            }
            previous_end = Some(end);
        }
    }
    Ok(!paused
        && same_revision
        && now >= effective
        && intervals.is_none_or(|ranges| ranges.iter().any(|&(a, b)| a <= now && now < b)))
}

/// P6: outstanding obligation to accepted live-owner set, represented synthetically.
pub fn accounted(
    outstanding: &BTreeSet<String>,
    ownership: &BTreeMap<String, BTreeSet<String>>,
    live_owners: &BTreeSet<String>,
) -> bool {
    outstanding.len() == ownership.len()
        && outstanding.iter().all(|id| {
            ownership.get(id).is_some_and(|owners| {
                owners.len() == 1 && owners.iter().all(|owner| live_owners.contains(owner))
            })
        })
}

/// Full disposal requires absence of obligations AND subscriptions.
pub fn may_terminate(obligations: usize, subscriptions: usize) -> bool {
    obligations == 0 && subscriptions == 0
}

#[cfg(test)]
mod tests;
