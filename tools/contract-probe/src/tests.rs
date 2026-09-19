// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

fn lit(n: i64) -> Box<Expr> {
    Box::new(Expr::Literal(Value::Integer(n)))
}

#[test]
fn canonical_integer_boundaries_and_spellings() {
    for n in [i64::MIN, -1, 0, 1, i64::MAX] {
        assert_eq!(integer(&n.to_string()), Ok(n));
    }
    for text in ["", "-", "+1", "00", "01", "-0", "1.0", "1e2", " 1", "１"] {
        assert_eq!(integer(text), Err(Error::Encoding), "{text}");
    }
    for text in ["9223372036854775808", "-9223372036854775809"] {
        assert_eq!(integer(text), Err(Error::Range));
    }
}

#[test]
fn decimals_never_round_or_overflow() {
    assert_eq!(rescale(123, 2, 4), Ok(12300));
    assert_eq!(rescale(-12300, 4, 2), Ok(-123));
    assert_eq!(rescale(12301, 4, 2), Err(Error::Inexact));
    assert_eq!(rescale(i64::MAX, 0, 1), Err(Error::Range));
    assert_eq!(rescale(1, 19, 0), Err(Error::Scale));
    for from in 0..=18 {
        for to in from..=18 {
            let result = rescale(1, from, to).unwrap();
            assert_eq!(rescale(result, to, from), Ok(1));
        }
    }
}

#[test]
fn dead_branches_are_typed_and_dependencies_cannot_hide() {
    let condition = || Box::new(Expr::Literal(Value::Boolean(true)));
    let snapshot = BTreeMap::from([("secret".into(), Value::Integer(7))]);
    let expression = Expr::If(condition(), lit(1), Box::new(Expr::Read("secret".into())));
    assert_eq!(
        evaluate(&expression, &snapshot, &BTreeSet::new()),
        Err(Error::Dependencies)
    );
    let reads = BTreeSet::from(["secret".into()]);
    assert_eq!(
        evaluate(&expression, &snapshot, &reads),
        Ok(Value::Integer(1))
    );
    assert_eq!(
        evaluate(&expression, &BTreeMap::new(), &reads),
        Err(Error::Unavailable)
    );
    let bad = Expr::If(condition(), lit(1), condition());
    assert_eq!(
        evaluate(&bad, &snapshot, &BTreeSet::new()),
        Err(Error::Type)
    );
    let overflow = Expr::Add(lit(i64::MAX), lit(1));
    assert_eq!(
        evaluate(&overflow, &snapshot, &BTreeSet::new()),
        Err(Error::Range)
    );
    let safe = Expr::If(condition(), lit(1), Box::new(overflow));
    assert_eq!(
        evaluate(&safe, &snapshot, &BTreeSet::new()),
        Ok(Value::Integer(1))
    );
}

#[test]
fn evaluation_bounds_precede_recursive_execution() {
    let mut e = Expr::Literal(Value::Integer(0));
    for _ in 0..65 {
        e = Expr::Add(Box::new(e), lit(0));
    }
    assert_eq!(
        evaluate(&e, &BTreeMap::new(), &BTreeSet::new()),
        Err(Error::Budget)
    );
    fn tree(depth: usize) -> Expr {
        if depth == 0 {
            return Expr::Literal(Value::Integer(0));
        }
        Expr::Add(Box::new(tree(depth - 1)), Box::new(tree(depth - 1)))
    }
    assert_eq!(
        evaluate(&tree(13), &BTreeMap::new(), &BTreeSet::new()),
        Err(Error::Budget)
    );
}

#[test]
fn completion_requires_current_authority_and_exact_correlation() {
    let mut c = Completion {
        assigned: true,
        accepted: true,
        terminal: false,
        same_principal: true,
        same_input_revision: true,
        authority_now: true,
        duplicate: false,
    };
    assert!(accept_completion(&c));
    c.authority_now = false;
    assert!(!accept_completion(&c));
    c.authority_now = true;
    c.same_input_revision = false;
    assert!(!accept_completion(&c));
    c.same_input_revision = true;
    c.duplicate = true;
    assert!(!accept_completion(&c));
    c.duplicate = false;
    c.same_principal = false;
    assert!(!accept_completion(&c));
}

#[test]
fn retry_needs_evidence_not_just_attempt_budget() {
    let mut r = Retry {
        attempts: 1,
        max_attempts: 3,
        eligible_fault: true,
        delay_elapsed: true,
        authority_now: true,
        cancelled: false,
        succeeded: false,
        unresolved: true,
        stable_key_supported: false,
        same_request: true,
        confirmed_no_effect: false,
    };
    assert_eq!(may_retry(&r), Ok(false));
    r.stable_key_supported = true;
    assert_eq!(may_retry(&r), Ok(true));
    r.same_request = false;
    assert_eq!(may_retry(&r), Ok(false));
    r.confirmed_no_effect = true;
    assert_eq!(may_retry(&r), Ok(false));
    r.same_request = true;
    assert_eq!(may_retry(&r), Ok(true));
    r.cancelled = true;
    assert_eq!(may_retry(&r), Ok(false));
    r.cancelled = false;
    r.succeeded = true;
    assert_eq!(may_retry(&r), Ok(false));
    r.succeeded = false;
    r.attempts = 3;
    assert_eq!(may_retry(&r), Ok(false));
    r.attempts = 0;
    assert_eq!(may_retry(&r), Err(Error::InvalidContext));
}

#[test]
fn timers_have_exact_boundaries_and_reject_stale_revisions() {
    assert_eq!(
        timer_eligible(10, 10, &vec![0; 10_001], false, true, None),
        Err(Error::Budget)
    );
    assert_eq!(
        timer_eligible(10, 10, &[], false, true, Some(&vec![(0, 1); 10_001])),
        Err(Error::Budget)
    );
    assert_eq!(timer_eligible(9, 10, &[], false, true, None), Ok(false));
    assert_eq!(timer_eligible(10, 10, &[], false, true, None), Ok(true));
    assert_eq!(timer_eligible(10, 10, &[], true, true, None), Ok(false));
    assert_eq!(timer_eligible(10, 10, &[], false, false, None), Ok(false));
    assert_eq!(timer_eligible(12, 10, &[3], false, true, None), Ok(false));
    assert_eq!(timer_eligible(13, 10, &[3], false, true, None), Ok(true));
    assert_eq!(
        timer_eligible(13, 10, &[-1], false, true, None),
        Err(Error::InvalidContext)
    );
    assert_eq!(
        timer_eligible(i64::MAX, i64::MAX, &[1], false, true, None),
        Err(Error::Range)
    );
    assert_eq!(
        timer_eligible(15, 10, &[], false, true, Some(&[(10, 15)])),
        Ok(false)
    );
    assert_eq!(
        timer_eligible(14, 10, &[], false, true, Some(&[(10, 15)])),
        Ok(true)
    );
    assert_eq!(
        timer_eligible(14, 10, &[], false, true, Some(&[(10, 15), (14, 20)])),
        Err(Error::InvalidContext)
    );
    assert_eq!(
        timer_eligible(14, 10, &[], false, true, Some(&[])),
        Ok(false)
    );
}

#[test]
fn closure_requires_total_unique_live_ownership_and_no_terminal_followup() {
    let outstanding = BTreeSet::from(["effect".into()]);
    let live = BTreeSet::from(["reconciliation".into(), "other".into()]);
    let mut ownership =
        BTreeMap::from([("effect".into(), BTreeSet::from(["reconciliation".into()]))]);
    assert!(accounted(&outstanding, &ownership, &live));
    assert!(!accounted(&outstanding, &ownership, &BTreeSet::new()));
    ownership.get_mut("effect").unwrap().insert("other".into());
    assert!(!accounted(&outstanding, &ownership, &live));
    assert!(!accounted(&outstanding, &BTreeMap::new(), &live));
    assert!(!may_terminate(0, 1));
    assert!(!may_terminate(1, 0));
    assert!(may_terminate(0, 0));
}
