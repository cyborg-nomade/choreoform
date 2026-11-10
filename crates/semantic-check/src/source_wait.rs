// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Borrowed actual-use declaration index, never a subscription or accepted fact.
use super::*;
pub(crate) struct SourceWaits<'a> {
    b: &'a Value,
    sites: BTreeMap<&'a str, Option<&'a str>>,
    writers: BTreeMap<&'a str, Option<&'a str>>,
}

impl<'a> SourceWaits<'a> {
    pub(crate) fn new(b: &'a Value, budget: &mut Budget) -> Result<Self> {
        let mut sites = BTreeMap::new();
        let mut writers = BTreeMap::new();
        for (id, n) in object(&b["nodes"])? {
            budget.tick(0)?;
            for cell in object(&n["writes"])?.keys() {
                budget.charge(2)?;
                writers
                    .entry(cell.as_str())
                    .and_modify(|writer| *writer = None)
                    .or_insert(Some(id.as_str()));
            }
            if n["kind"] == "wait" {
                budget.charge(2)?;
                sites
                    .entry(text(&n["policy"])?)
                    .and_modify(|site| *site = None)
                    .or_insert(Some(id.as_str()));
            }
        }
        Ok(Self { b, sites, writers })
    }
    /// Static type/provider/capture checking has already run on this definition.
    pub(crate) fn capture(
        &self,
        scope: &str,
        origin: &str,
        budget: &mut Budget,
    ) -> Result<&'a str> {
        budget.charge(4)?;
        let b = self.b;
        let capability = &b["capabilities"][text(&b["nodes"][origin]["target"])?];
        let policy = text(&b["policies"][text(&capability["effects"])?]["body"]["reconciliation"])?;
        let site = self
            .sites
            .get(policy)
            .copied()
            .flatten()
            .ok_or_else(|| err("retained-request-source-wait"))?;
        let wait = &b["nodes"][site];
        require(
            wait["scope"] == scope
                && wait["capture"]["origin"] == origin
                && b["policies"][policy]["body"]["timer"].is_null(),
            "retained-request-source-wait",
        )?;
        let result = text(&wait["capture"]["result"])?;
        require(
            b["data"][result]["scope"] == scope
                && b["data"][result].get("initial").is_none()
                && self.writers.get(result) == Some(&Some(site)),
            "retained-request-source-result",
        )?;
        Ok(site)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn borrowed_index_and_ambiguous_actual_use_share_one_cumulative_allowance() {
        let (e, _) = crate::owned_continuation::tests::fixture();
        let mut b = e.document()["body"].clone();
        let mut exhausted = Budget { work: 1_000_000 };
        assert!(matches!(SourceWaits::new(&b,&mut exhausted),Err(e) if e.code=="budget"));
        let mut budget = Budget::default();
        let index = SourceWaits::new(&b, &mut budget).unwrap();
        assert_eq!(
            index.capture("recovery", "origin", &mut budget).unwrap(),
            "capture_wait"
        );
        let before = budget.work;
        index.capture("recovery", "origin", &mut budget).unwrap();
        assert_eq!(budget.work - before, 4);
        let mut exhausted = Budget { work: 999_998 };
        assert_eq!(
            index
                .capture("recovery", "origin", &mut exhausted)
                .unwrap_err()
                .code,
            "budget"
        );
        // Private index evidence only; public admission rejects these duplicate
        // declaration users earlier, at the existing lifecycle check.
        for id in ["duplicate_one", "duplicate_two"] {
            b["nodes"][id] = b["nodes"]["capture_wait"].clone();
        }
        let index = SourceWaits::new(&b, &mut Budget::default()).unwrap();
        assert_eq!(
            index
                .capture("recovery", "origin", &mut Budget::default())
                .unwrap_err()
                .code,
            "retained-request-source-wait"
        );
    }
}
