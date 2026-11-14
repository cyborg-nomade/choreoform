// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
//! Exact local suite descriptions, never providers or host authorization.
use super::*;
use std::collections::BTreeSet;
const BUILTINS: [Resource<'static>; 13] = [
    Resource {
        id: "urn:choreoform:semantics:adr-0008",
        revision: "sha256:0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141",
        bytes: include_bytes!(
            "../../../../docs/ir/contracts/sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt"
        ),
    },
    Resource {
        id: "urn:choreoform:contracts:core-profile:0.1.0",
        revision: "sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58",
        bytes: include_bytes!(
            "../../../../docs/ir/contracts/sha256-26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58.txt"
        ),
    },
    Resource {
        id: "urn:choreoform:contracts:core-profile:0.2.0",
        revision: "sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623",
        bytes: include_bytes!(
            "../../../../docs/ir/contracts/sha256-5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623.txt"
        ),
    },
    Resource {
        id: "urn:choreoform:contracts:core-profile:0.3.0",
        revision: "sha256:30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3",
        bytes: include_bytes!(
            "../../../../docs/ir/contracts/sha256-30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3.txt"
        ),
    },
    Resource {
        id: "urn:choreoform:contracts:core-profile:0.4.0",
        revision: "sha256:0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3",
        bytes: include_bytes!(
            "../../../../docs/ir/contracts/sha256-0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3.txt"
        ),
    },
    Resource {
        id: "urn:choreoform:contracts:core-profile:0.5.0",
        revision: "sha256:50007ac05e5e71df91bb9997a40a06c7a8993df04b1a673421e4a987014299d7",
        bytes: include_bytes!(
            "../../../../docs/ir/contracts/sha256-50007ac05e5e71df91bb9997a40a06c7a8993df04b1a673421e4a987014299d7.txt"
        ),
    },
    Resource {
        id: "urn:choreoform:contracts:core-profile:0.6.0",
        revision: "sha256:27bce3fde9a0393abb6bf760a8588af9268de65069b73f4cb946144f5d30cb85",
        bytes: include_bytes!(
            "../../../../docs/ir/contracts/sha256-27bce3fde9a0393abb6bf760a8588af9268de65069b73f4cb946144f5d30cb85.txt"
        ),
    },
    Resource {
        id: "urn:choreoform:contracts:core-profile:0.7.0",
        revision: "sha256:e286e54441b3d785ae823eef359c12d64fb8405829c70709227282e5af62f54b",
        bytes: include_bytes!(
            "../../../../docs/ir/contracts/sha256-e286e54441b3d785ae823eef359c12d64fb8405829c70709227282e5af62f54b.txt"
        ),
    },
    Resource {
        id: "urn:choreoform:contracts:core-profile:0.8.0",
        revision: "sha256:edc8195ba0c31536ab2cdc95089e8aecc3ff402489639b4435277f0a38e7b603",
        bytes: include_bytes!(
            "../../../../docs/ir/contracts/sha256-edc8195ba0c31536ab2cdc95089e8aecc3ff402489639b4435277f0a38e7b603.txt"
        ),
    },
    Resource {
        id: "urn:choreoform:contracts:core-profile:0.9.0",
        revision: "sha256:5faaf24217bcc5644a9ce5975b2c6ad7074d0197d39a73ab648d6cb3a9ced87f",
        bytes: include_bytes!(
            "../../../../docs/ir/contracts/sha256-5faaf24217bcc5644a9ce5975b2c6ad7074d0197d39a73ab648d6cb3a9ced87f.txt"
        ),
    },
    Resource {
        id: "urn:choreoform:contracts:core-profile:0.10.0",
        revision: "sha256:74c3d5bfe2a880259bc13f521127aa1dd84977ae9e6f83b339e3a0629964e60f",
        bytes: include_bytes!(
            "../../../../docs/ir/contracts/sha256-74c3d5bfe2a880259bc13f521127aa1dd84977ae9e6f83b339e3a0629964e60f.txt"
        ),
    },
    Resource {
        id: "urn:choreoform:contracts:core-profile:0.11.0",
        revision: "sha256:246bf3835eef25f8adf76bce39e6b887715e098761e675f821bf745a8111d29e",
        bytes: include_bytes!(
            "../../../../docs/ir/contracts/sha256-246bf3835eef25f8adf76bce39e6b887715e098761e675f821bf745a8111d29e.txt"
        ),
    },
    Resource {
        id: "urn:choreoform:contracts:core-profile:0.12.0",
        revision: "sha256:73158731c33084fde009abcc86530926447902609fda9535f04daa7c768c2007",
        bytes: include_bytes!(
            "../../../../docs/ir/contracts/sha256-73158731c33084fde009abcc86530926447902609fda9535f04daa7c768c2007.txt"
        ),
    },
];
pub(super) fn registry<'a>(supplied: &[Resource<'a>]) -> AuthoringResult<Vec<Resource<'a>>> {
    let fail = || diagnostic("resource-integrity", AuthoringComponent::Resources, "");
    if supplied.len() > 128 {
        return Err(diagnostic(
            "registry-limit",
            AuthoringComponent::Resources,
            "",
        ));
    }
    let mut total = 0usize;
    let mut seen = BTreeSet::new();
    for row in supplied {
        total = total.checked_add(row.bytes.len()).ok_or_else(fail)?;
        if row.bytes.len() > MAX_BYTES || total > 8 * MAX_BYTES {
            return Err(diagnostic(
                "registry-limit",
                AuthoringComponent::Resources,
                "",
            ));
        }
        if !seen.insert((row.id, row.revision)) || digest(row.bytes) != row.revision {
            return Err(fail());
        }
    }
    let mut result: Vec<_> = BUILTINS
        .iter()
        .map(|r| Resource {
            id: r.id,
            revision: r.revision,
            bytes: r.bytes,
        })
        .collect();
    for row in supplied {
        if let Some(builtin) = BUILTINS
            .iter()
            .find(|r| r.id == row.id && r.revision == row.revision)
        {
            if builtin.bytes != row.bytes {
                return Err(fail());
            }
        } else {
            result.push(Resource {
                id: row.id,
                revision: row.revision,
                bytes: row.bytes,
            });
        }
    }
    Ok(result)
}
