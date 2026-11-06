// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;
use choreoform_semantic_check::CORE_V2;

#[test]
fn source_transfer_derives_receiver_authority_reads_in_both_profiles() {
    for modern in [false, true] {
        let (mut source, companion) = child_input_source();
        source = source.replace(
            "Keep \"Child input\" as text under \"Input access\".",
            "Keep \"Child input\" as text under \"Protection\".",
        );
        let (before, child) = source.split_once("Scope \"Child\"").unwrap();
        source=format!("{before}Scope \"Child\"{}",child.replace("Use cancellation \"Stop\"", "Use cancellation \"Child stop\"")
            .replace("End scope.","Rule for settlement \"Transfer\". Unfinished work transfer to \"Review example\". End policy. Rule for cancel \"Child stop\". Authority \"Reviewer\". For children use \"Transfer\". End policy. End scope."));
        source=source.replace("Rule for cancel \"Stop\".\n    Authority \"Reviewer\".","Rule for cancel \"Stop\".\n    Authority \"Receiver\".")
            .replace("End process.","Require actor \"Receiver\" under \"Receiver requirement\". Rule for actor \"Receiver requirement\". Permit when ((the value of \"Audit basis\") equals (0)). End policy. End process.");
        let mut ledger: Value = serde_json::from_slice(&companion).unwrap();
        ledger["declarations"].as_array_mut().unwrap().extend([
            json!({"id":"receiver_actor","kind":"actor","scope":"scope_review","name":"Receiver"}),
            json!({"id":"receiver_requirement","kind":"policy","scope":"scope_review","name":"Receiver requirement"}),
            json!({"id":"child_transfer","kind":"policy","scope":"scope_child","name":"Transfer"}),
            json!({"id":"child_cancel","kind":"policy","scope":"scope_child","name":"Child stop"}),
        ]);
        let mut r = resources();
        if modern {
            source = source.replace("\"0.2.0\"", "\"0.3.0\"");
            ledger["version"] = "0.2.0".into();
            ledger["profile"] = "0.3.0".into();
            ledger["contracts"]["core"]["contract"] = CORE_V2.0.into();
            ledger["contracts"]["core"]["digest"] = CORE_V2.1.into();
            r.push(Resource{id:CORE_V2.0,revision:CORE_V2.1,bytes:include_bytes!("../../../../docs/ir/contracts/sha256-5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623.txt")});
        }
        let raw = serde_json::to_vec(&ledger).unwrap();
        let package = Package::parse(source.as_bytes(), &raw).unwrap();
        let candidate = package.lower(&r).unwrap();
        for policy in ["child_transfer", "child_cancel"] {
            assert_eq!(
                candidate.document()["body"]["policies"][policy]["body"]["reads"],
                json!({"review_audit":true})
            );
        }
        assert_eq!(
            candidate.document()["body"]["nodes"]["child_done"]["reads"],
            json!({"review_audit":true})
        );
        candidate.check_protection(&r).unwrap();
        let (formatted, companion) = package.formatted().unwrap();
        let roundtrip = Package::parse(formatted.as_bytes(), companion.as_bytes())
            .unwrap()
            .lower(&r)
            .unwrap();
        assert_eq!(roundtrip.document(), candidate.document());
        assert_eq!(
            roundtrip
                .check_protection(&r)
                .unwrap()
                .control()
                .covered_nodes(),
            candidate
                .check_protection(&r)
                .unwrap()
                .control()
                .covered_nodes()
        );
    }
}
