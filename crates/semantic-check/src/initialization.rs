// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Immutable declaration metadata only; never initializer values or authority.
use super::*;
pub(crate) type Index<'a> = BTreeMap<&'a str, Vec<&'a str>>;
pub(crate) fn by_scope<'a>(b: &'a Value, budget: &mut Budget) -> Result<Index<'a>> {
    let mut index = Index::new();
    for (id, data) in object(&b["data"])? {
        budget.tick(0)?;
        if data.get("initial").is_some() {
            budget.tick(0)?;
            index
                .entry(text(&data["scope"])?)
                .or_default()
                .push(id.as_str());
        }
    }
    Ok(index)
}
