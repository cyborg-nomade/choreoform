// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use choreoform_authoring_frontend::Package;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};
use choreoform_semantic_check::{
    CORE, SEMANTICS, check, check_concurrency, check_control, check_invalidation, check_protection,
};
use serde_json::{Value, json};

#[path = "control/settlement.rs"]
mod settlement;

const ROOT: &str = "scope_terminal";
const INTEGER_MARKER: &str = "424242";
fn resources() -> Vec<Resource<'static>> {
    vec![
        Resource {
            id: SEMANTICS.0,
            revision: SEMANTICS.1,
            bytes: include_bytes!(
                "../../../docs/ir/contracts/sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt"
            ),
        },
        Resource {
            id: CORE.0,
            revision: CORE.1,
            bytes: include_bytes!(
                "../../../docs/ir/contracts/sha256-26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58.txt"
            ),
        },
    ]
}
fn base() -> Value {
    let candidate = Package::parse(
        include_bytes!("../../../docs/authoring/examples/terminal.choreo"),
        include_bytes!("../../../docs/authoring/examples/terminal.bindings.json"),
    )
    .unwrap()
    .lower(&resources())
    .unwrap();
    let mut b = candidate.document()["body"].clone();
    b["policies"]["protect"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"protection","reads":{},"permit":{"op":"literal","type":{"kind":"boolean"},"value":true}}});
    expr(
        &mut b,
        "zero",
        json!({"kind":"integer"}),
        json!({"op":"literal","type":{"kind":"integer"},"value":INTEGER_MARKER}),
        json!({}),
    );
    expr(
        &mut b,
        "yes",
        json!({"kind":"boolean"}),
        json!({"op":"literal","type":{"kind":"boolean"},"value":true}),
        json!({}),
    );
    b
}
fn envelope(b: Value) -> DefinitionEnvelope {
    DefinitionEnvelope::from_parts(b, json!({})).unwrap()
}
fn pass(b: Value) {
    check_control(&envelope(b), &resources()).unwrap();
}
fn refuse(b: Value, expected: &str) {
    let e = envelope(b);
    assert_eq!(
        e.document()["body"]["expressions"]["zero"]["body"]["value"],
        INTEGER_MARKER
    );
    // These candidates deliberately pass the old payload stage.
    check(&e, &resources()).unwrap();
    let error = check_control(&e, &resources()).unwrap_err();
    assert_eq!(error.code, expected, "{error:?}");
    assert!(!format!("{error:?}").contains(INTEGER_MARKER));
}
fn cell(b: &mut Value, id: &str, scope: &str, initialized: bool) {
    b["data"][id] = json!({"scope":scope,"type":{"dialect":"core","body":{"kind":"integer"}},"protection":{"policy":"protect","sensitivity":"internal","purposes":{"test":true},"participants":{},"capabilities":{}},"invalidates":{}});
    if initialized {
        b["data"][id]["initial"] = "zero".into();
    }
}
fn expr(b: &mut Value, id: &str, ty: Value, body: Value, reads: Value) {
    b["expressions"][id] = json!({"scope":ROOT,"dialect":"core","resultType":{"dialect":"core","body":ty},"parameters":{},"reads":reads,"body":body});
}
fn read(b: &mut Value, id: &str, cell: &str) {
    expr(
        b,
        id,
        json!({"kind":"integer"}),
        json!({"op":"read","cell":cell}),
        json!({cell:true}),
    );
}
fn compute(b: &mut Value, id: &str, scope: &str, assignments: Value, reads: Value) {
    if assignments.as_object().unwrap().is_empty() {
        b["nodes"][id] = json!({"kind":"decision","scope":scope,"guards":{"next":"yes"},"default":null,"reads":reads,"writes":{},"outcomes":{"next":true}});
        return;
    }
    let writes: serde_json::Map<_, _> = assignments
        .as_object()
        .unwrap()
        .keys()
        .map(|s| (s.clone(), true.into()))
        .collect();
    b["nodes"][id] = json!({"kind":"compute","scope":scope,"assignments":assignments,"reads":reads,"writes":writes,"outcomes":{"next":true}});
}
fn flow(b: &mut Value, id: &str, source: &str, outcome: &str, target: &str) {
    b["flows"][id] = json!({"source":source,"outcome":outcome,"target":target});
}
fn entry(b: &mut Value, id: &str) {
    b["scopes"][ROOT]["entry"] = id.into();
}
fn child(b: &mut Value, id: &str) {
    let mut scope = b["scopes"][ROOT].clone();
    scope["parent"] = ROOT.into();
    scope["entry"] = format!("{id}_done").into();
    scope["inputs"] = json!({});
    scope["outputs"] = json!({});
    b["scopes"][id] = scope;
    b["nodes"][format!("{id}_done")] = json!({"kind":"finish","scope":id,"reads":{},"writes":{},"outcomes":{},"outcome":"complete"});
}
fn invoke(b: &mut Value, id: &str, child: &str, outputs: Value) {
    let writes: serde_json::Map<_, _> = outputs
        .as_object()
        .unwrap()
        .values()
        .map(|s| (s.as_str().unwrap().to_owned(), true.into()))
        .collect();
    b["nodes"][id] = json!({"kind":"invoke","scope":ROOT,"body":child,"inputs":{},"outputs":outputs,"reads":{},"writes":writes,"outcomes":{"complete":true}});
    flow(b, &format!("flow_{id}"), id, "complete", "node_done");
    entry(b, id);
}
fn branch() -> Value {
    let mut b = base();
    cell(&mut b, "answer", ROOT, false);
    b["scopes"][ROOT]["outputs"] = json!({"answer":"answer"});
    b["nodes"]["choose"] = json!({"kind":"decision","scope":ROOT,"guards":{"left":"yes"},"default":"right","reads":{},"writes":{},"outcomes":{"left":true,"right":true}});
    compute(
        &mut b,
        "write_left",
        ROOT,
        json!({"answer":"zero"}),
        json!({}),
    );
    compute(
        &mut b,
        "write_right",
        ROOT,
        json!({"answer":"zero"}),
        json!({}),
    );
    entry(&mut b, "choose");
    flow(&mut b, "choose_left", "choose", "left", "write_left");
    flow(&mut b, "choose_right", "choose", "right", "write_right");
    flow(&mut b, "left_done", "write_left", "next", "node_done");
    flow(&mut b, "right_done", "write_right", "next", "node_done");
    b
}
fn parallel() -> Value {
    let mut b = base();
    child(&mut b, "child_a");
    child(&mut b, "child_b");
    b["nodes"]["fork"] = json!({"kind":"split","scope":ROOT,"children":{"a":"child_a","b":"child_b"},"join":"barrier","reads":{},"writes":{},"outcomes":{}});
    b["nodes"]["barrier"] = json!({"kind":"join","scope":ROOT,"source":"fork","predicate":{"op":"all","outcomes":{"complete":true}},"remaining":"policy_settle","reads":{},"writes":{},"outcomes":{"next":true}});
    flow(&mut b, "barrier_done", "barrier", "next", "node_done");
    entry(&mut b, "fork");
    b
}
fn fanout() -> Value {
    let mut b = parallel();
    cell(&mut b, "item_cell", "child_a", false);
    b["scopes"]["child_a"]["inputs"] = json!({"item":"item_cell"});
    expr(
        &mut b,
        "collection",
        json!({"kind":"list","element":{"kind":"integer"}}),
        json!({"op":"literal","type":{"kind":"list","element":{"kind":"integer"}},"value":["0"]}),
        json!({}),
    );
    expr(
        &mut b,
        "key",
        json!({"kind":"text"}),
        json!({"op":"literal","type":{"kind":"text"},"value":"only-item"}),
        json!({}),
    );
    b["expressions"]["key"]["parameters"] =
        json!({"item":{"dialect":"core","body":{"kind":"integer"}}});
    b["policies"]["membership"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"membership","reads":{},"removal":"policy_settle","changed":"invalidate"}});
    b["nodes"]["fork"] = json!({"kind":"fanout","scope":ROOT,"collection":"collection","itemKey":"key","body":"child_a","item":"item_cell","join":"barrier","seal":"yes","changes":"membership","reads":{},"writes":{},"outcomes":{}});
    b["nodes"]["child_a_done"]["reads"] = json!({"item_cell":true});
    b
}

#[test]
fn original_source_examples_and_immutable_coverage_report() {
    let c = Package::parse(
        include_bytes!("../../../docs/authoring/examples/review.choreo"),
        include_bytes!("../../../docs/authoring/examples/review.bindings.json"),
    )
    .unwrap()
    .lower(&resources())
    .unwrap();
    let before = c.document().clone();
    let report = c.check_control(&resources()).unwrap();
    assert_eq!(report.revision(), before["revision"]);
    assert_eq!(report.covered_nodes().len(), 2);
    assert!(report.uncovered_nodes().is_empty());
    assert!(report.required_inputs().is_empty());
    assert!(!report.remaining().is_empty());
    assert_eq!(c.document(), &before);
    pass(base());
}
#[test]
fn every_ordinary_outcome_has_one_flow_including_unreachable_nodes() {
    let b = branch();
    pass(b.clone());
    let mut missing = b.clone();
    missing["flows"]
        .as_object_mut()
        .unwrap()
        .remove("right_done");
    refuse(missing, "flow-missing");
    let mut unreachable = b.clone();
    compute(
        &mut unreachable,
        "unreachable_write",
        ROOT,
        json!({"answer":"zero"}),
        json!({}),
    );
    refuse(unreachable, "flow-missing");
    let mut duplicate = b;
    duplicate["flows"]["duplicate"] = duplicate["flows"]["right_done"].clone();
    refuse(duplicate, "flow-duplicate");
}
#[test]
fn reachable_and_unreachable_graph_cycles_refuse() {
    let mut b = base();
    compute(&mut b, "loop_node", ROOT, json!({}), json!({}));
    flow(&mut b, "loop_edge", "loop_node", "next", "loop_node");
    refuse(b.clone(), "flow-cycle");
    entry(&mut b, "loop_node");
    refuse(b, "flow-cycle");
}
#[test]
fn branch_intersections_do_not_prune_constant_guards() {
    let mut b = branch();
    compute(&mut b, "write_right", ROOT, json!({}), json!({}));
    refuse(b, "output-availability-unproved");
}
#[test]
fn every_finish_must_supply_outputs() {
    let mut b = branch();
    b["nodes"]["early_done"] = b["nodes"]["node_done"].clone();
    b["flows"]["choose_right"]["target"] = "early_done".into();
    refuse(b, "output-availability-unproved");
}
#[test]
fn reads_precede_all_simultaneous_writes() {
    let mut b = base();
    cell(&mut b, "first", ROOT, false);
    cell(&mut b, "second", ROOT, false);
    read(&mut b, "read_first", "first");
    compute(
        &mut b,
        "assign",
        ROOT,
        json!({"first":"zero","second":"read_first"}),
        json!({"first":true}),
    );
    entry(&mut b, "assign");
    flow(&mut b, "assign_done", "assign", "next", "node_done");
    refuse(b.clone(), "data-availability-unproved");
    b["data"]["first"]["initial"] = "zero".into();
    pass(b);
}
#[test]
fn sequential_writes_and_explicit_input_assumptions() {
    let mut b = base();
    cell(&mut b, "first", ROOT, false);
    cell(&mut b, "second", ROOT, false);
    read(&mut b, "read_first", "first");
    compute(
        &mut b,
        "assign",
        ROOT,
        json!({"second":"read_first"}),
        json!({"first":true}),
    );
    entry(&mut b, "assign");
    flow(&mut b, "assign_done", "assign", "next", "node_done");
    b["scopes"][ROOT]["inputs"] = json!({"argument":"first"});
    b["scopes"][ROOT]["outputs"] = json!({"answer":"second"});
    let report = check_control(&envelope(b.clone()), &resources()).unwrap();
    assert_eq!(
        report.required_inputs().iter().collect::<Vec<_>>(),
        vec!["first"]
    );
    b["scopes"][ROOT]["inputs"] = json!({});
    compute(&mut b, "seed", ROOT, json!({"first":"zero"}), json!({}));
    flow(&mut b, "seed_assign", "seed", "next", "assign");
    entry(&mut b, "seed");
    pass(b);
}
#[test]
fn conservative_extra_reads_still_require_initialized_data() {
    let mut b = base();
    cell(&mut b, "unused", ROOT, false);
    b["nodes"]["node_done"]["reads"] = json!({"unused":true});
    refuse(b, "data-availability-unproved");
}
#[test]
fn initializers_use_entry_inputs_not_guessed_local_order() {
    let mut b = base();
    cell(&mut b, "first", ROOT, true);
    cell(&mut b, "second", ROOT, false);
    read(&mut b, "read_first", "first");
    b["data"]["second"]["initial"] = "read_first".into();
    refuse(b.clone(), "initializer-availability-unproved");
    b["data"]["first"]
        .as_object_mut()
        .unwrap()
        .remove("initial");
    b["scopes"][ROOT]["inputs"] = json!({"argument":"first"});
    pass(b);
}
#[test]
fn invoke_outputs_and_ancestor_writes_are_transferred_after_completion() {
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "local", "child", true);
    cell(&mut b, "target", ROOT, false);
    cell(&mut b, "ancestor", ROOT, false);
    b["scopes"]["child"]["outputs"] = json!({"value":"local"});
    compute(
        &mut b,
        "child_assign",
        "child",
        json!({"ancestor":"zero"}),
        json!({}),
    );
    b["scopes"]["child"]["entry"] = "child_assign".into();
    flow(&mut b, "child_flow", "child_assign", "next", "child_done");
    invoke(&mut b, "call", "child", json!({"value":"target"}));
    b["scopes"][ROOT]["outputs"] = json!({"result":"target","lexical":"ancestor"});
    pass(b.clone());
    b["data"]["local"]
        .as_object_mut()
        .unwrap()
        .remove("initial");
    refuse(b, "output-availability-unproved");
}
#[test]
fn invoke_checks_fresh_locals_at_each_occurrence() {
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "local", "child", false);
    b["nodes"]["child_done"]["reads"] = json!({"local":true});
    invoke(&mut b, "call", "child", json!({}));
    refuse(b, "data-availability-unproved");
}
#[test]
fn repeated_calls_recheck_context_and_nested_calls_export_only_ancestors() {
    let mut b = base();
    child(&mut b, "child");
    child(&mut b, "grandchild");
    b["scopes"]["grandchild"]["parent"] = "child".into();
    cell(&mut b, "ancestor", ROOT, false);
    cell(&mut b, "local", "grandchild", true);
    compute(
        &mut b,
        "grandchild_assign",
        "grandchild",
        json!({"ancestor":"zero"}),
        json!({"local":true}),
    );
    b["scopes"]["grandchild"]["entry"] = "grandchild_assign".into();
    flow(
        &mut b,
        "grandchild_flow",
        "grandchild_assign",
        "next",
        "grandchild_done",
    );
    invoke(&mut b, "nested_call", "grandchild", json!({}));
    b["nodes"]["nested_call"]["scope"] = "child".into();
    b["flows"]["flow_nested_call"]["target"] = "child_done".into();
    b["scopes"]["child"]["entry"] = "nested_call".into();
    invoke(&mut b, "first_call", "child", json!({}));
    invoke(&mut b, "second_call", "child", json!({}));
    entry(&mut b, "first_call");
    b["flows"]["flow_first_call"]["target"] = "second_call".into();
    b["nodes"]["second_call"]["reads"] = json!({"ancestor":true});
    b["scopes"][ROOT]["outputs"] = json!({"result":"ancestor"});
    pass(b.clone());
    b["data"]["local"]
        .as_object_mut()
        .unwrap()
        .remove("initial");
    refuse(b, "data-availability-unproved");
}
#[test]
fn child_completion_summary_intersects_all_finishes() {
    let mut b = branch();
    // Move the branching graph into a called scope, writing a root-owned cell.
    child(&mut b, "child");
    b["scopes"][ROOT]["outputs"] = json!({"answer":"answer"});
    for id in ["choose", "write_left", "write_right"] {
        b["nodes"][id]["scope"] = "child".into();
    }
    b["flows"]["left_done"]["target"] = "child_done".into();
    b["flows"]["right_done"]["target"] = "child_done".into();
    b["scopes"]["child"]["entry"] = "choose".into();
    invoke(&mut b, "call", "child", json!({}));
    pass(b.clone());
    compute(&mut b, "write_right", "child", json!({}), json!({}));
    refuse(b, "output-availability-unproved");
}
#[test]
fn invoke_input_cells_seed_only_after_caller_dependencies() {
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "local", "child", false);
    cell(&mut b, "argument", ROOT, false);
    read(&mut b, "read_argument", "argument");
    b["scopes"]["child"]["inputs"] = json!({"value":"local"});
    b["nodes"]["child_done"]["reads"] = json!({"local":true});
    invoke(&mut b, "call", "child", json!({}));
    b["nodes"]["call"]["inputs"] = json!({"value":"read_argument"});
    b["nodes"]["call"]["reads"] = json!({"argument":true});
    refuse(b.clone(), "data-availability-unproved");
    b["data"]["argument"]["initial"] = "zero".into();
    pass(b);
}
#[test]
fn input_and_output_aliases_and_extra_invoke_writes_refuse() {
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "local", "child", true);
    cell(&mut b, "target", ROOT, false);
    cell(&mut b, "extra", ROOT, false);
    b["scopes"]["child"]["outputs"] = json!({"a":"local","b":"local"});
    invoke(&mut b, "call", "child", json!({"a":"target","b":"target"}));
    refuse(b.clone(), "output-target-alias");
    b["nodes"]["call"]["outputs"]["b"] = "extra".into();
    b["nodes"]["call"]["writes"]["extra"] = true.into();
    pass(b.clone());
    b["nodes"]["call"]["outputs"]
        .as_object_mut()
        .unwrap()
        .remove("b");
    b["scopes"]["child"]["outputs"]
        .as_object_mut()
        .unwrap()
        .remove("b");
    refuse(b, "invoke-write-set");
    let mut b = base();
    cell(&mut b, "argument", ROOT, false);
    b["scopes"][ROOT]["inputs"] = json!({"a":"argument","b":"argument"});
    refuse(b, "input-cell-alias");
}
#[test]
fn repeat_has_a_zero_iteration_path() {
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "result", ROOT, false);
    compute(
        &mut b,
        "child_assign",
        "child",
        json!({"result":"zero"}),
        json!({}),
    );
    b["scopes"]["child"]["entry"] = "child_assign".into();
    flow(&mut b, "child_flow", "child_assign", "next", "child_done");
    b["nodes"]["loop_node"] = json!({"kind":"repeat","scope":ROOT,"body":"child","condition":"yes","reads":{},"writes":{},"outcomes":{"next":true}});
    entry(&mut b, "loop_node");
    flow(&mut b, "loop_done", "loop_node", "next", "node_done");
    b["scopes"][ROOT]["outputs"] = json!({"result":"result"});
    refuse(b.clone(), "output-availability-unproved");
    b["data"]["result"]["initial"] = "zero".into();
    pass(b);
}
#[test]
fn reciprocal_joins_and_population_tokens() {
    let b = parallel();
    pass(b.clone());
    let mut wrong = b.clone();
    wrong["nodes"]["barrier"]["predicate"]["outcomes"] = json!({"absent":true});
    refuse(wrong, "join-outcome");
    let mut wrong = b.clone();
    wrong["nodes"]["barrier"]["source"] = "node_done".into();
    refuse(wrong, "join-pair");
    let mut wrong = b;
    wrong["nodes"]["fork"]["join"] = "node_done".into();
    refuse(wrong, "join-pair");
    let mut orphan = parallel();
    orphan["nodes"]["orphan_join"] = orphan["nodes"]["barrier"].clone();
    flow(
        &mut orphan,
        "orphan_done",
        "orphan_join",
        "next",
        "node_done",
    );
    refuse(orphan, "join-pair");
}
#[test]
fn join_activation_cannot_bypass_its_source() {
    let mut b = parallel();
    entry(&mut b, "barrier");
    refuse(b, "join-activation");
    let mut b = parallel();
    compute(&mut b, "bypass", ROOT, json!({}), json!({}));
    flow(&mut b, "bypass_join", "bypass", "next", "barrier");
    refuse(b, "join-activation");
}
#[test]
fn parallel_children_cannot_supply_sibling_or_join_initialization() {
    let mut b = parallel();
    cell(&mut b, "shared", ROOT, false);
    compute(
        &mut b,
        "child_assign",
        "child_a",
        json!({"shared":"zero"}),
        json!({}),
    );
    b["scopes"]["child_a"]["entry"] = "child_assign".into();
    flow(&mut b, "child_flow", "child_assign", "next", "child_a_done");
    b["nodes"]["child_b_done"]["reads"] = json!({"shared":true});
    refuse(b.clone(), "data-availability-unproved");
    b["nodes"]["child_b_done"]["reads"] = json!({});
    b["scopes"][ROOT]["outputs"] = json!({"result":"shared"});
    refuse(b.clone(), "output-availability-unproved");
    b["data"]["shared"]["initial"] = "zero".into();
    pass(b);
}
#[test]
fn fanout_item_is_initialized_but_child_writes_do_not_escape() {
    let mut b = fanout();
    pass(b.clone());
    cell(&mut b, "shared", ROOT, false);
    compute(
        &mut b,
        "child_assign",
        "child_a",
        json!({"shared":"zero"}),
        json!({"item_cell":true}),
    );
    b["scopes"]["child_a"]["entry"] = "child_assign".into();
    flow(&mut b, "child_flow", "child_assign", "next", "child_a_done");
    b["scopes"][ROOT]["outputs"] = json!({"result":"shared"});
    refuse(b, "output-availability-unproved");
}
#[test]
fn nested_join_predicates_validate_every_population_token_without_liveness_claims() {
    let mut b = parallel();
    b["nodes"]["barrier"]["predicate"] = json!({"op":"and","terms":{"one":{"op":"any","outcomes":{"complete":true}},"two":{"op":"or","terms":{"many":{"op":"atLeast","count":99,"outcomes":{"complete":true}}}}}});
    pass(b.clone());
    b["nodes"]["barrier"]["predicate"]["terms"]["two"]["terms"]["many"]["outcomes"] =
        json!({"unknown":true});
    refuse(b, "join-outcome");
}
#[test]
fn policy_dependencies_are_checked_before_the_transition() {
    let mut b = base();
    cell(&mut b, "policy_input", ROOT, false);
    read(&mut b, "policy_read", "policy_input");
    b["policies"]["policy_actor"]["body"]["permit"] = json!({"op":"eq","left":{"op":"read","cell":"policy_input"},"right":{"op":"literal","type":{"kind":"integer"},"value":"0"}});
    b["policies"]["policy_actor"]["body"]["reads"] = json!({"policy_input":true});
    b["policies"]["policy_stop"]["body"]["reads"] = json!({"policy_input":true});
    b["nodes"]["node_done"]["reads"] = json!({"policy_input":true});
    refuse(b.clone(), "data-availability-unproved");
    b["data"]["policy_input"]["initial"] = "zero".into();
    pass(b);
}
#[test]
fn unreachable_declarations_have_no_invented_entry_context() {
    let mut b = base();
    cell(&mut b, "unavailable", ROOT, false);
    compute(
        &mut b,
        "unreachable",
        ROOT,
        json!({}),
        json!({"unavailable":true}),
    );
    flow(
        &mut b,
        "unreachable_done",
        "unreachable",
        "next",
        "node_done",
    );
    child(&mut b, "uncalled");
    cell(&mut b, "uncalled_local", "uncalled", false);
    b["nodes"]["uncalled_done"]["reads"] = json!({"uncalled_local":true});
    let report = check_control(&envelope(b), &resources()).unwrap();
    assert!(report.uncovered_nodes().contains("unreachable"));
    assert!(report.uncovered_nodes().contains("uncalled_done"));
    assert!(!report.covered_scopes().contains("uncalled"));
    assert_eq!(report.covered_nodes().len(), 1);
}
fn chain(length: usize, cells: usize) -> Value {
    let mut b = base();
    for i in 0..cells {
        cell(&mut b, &format!("cell_{i:04}"), ROOT, true);
    }
    for i in 0..length {
        let id = format!("chain_{i:04}");
        compute(&mut b, &id, ROOT, json!({}), json!({}));
        let target = if i + 1 == length {
            "node_done".into()
        } else {
            format!("chain_{:04}", i + 1)
        };
        flow(&mut b, &format!("edge_{i:04}"), &id, "next", &target);
    }
    entry(&mut b, "chain_0000");
    b
}
#[test]
fn long_flat_dag_is_iterative_and_dense_state_work_refuses_boundedly() {
    pass(chain(1200, 0));
    refuse(chain(1000, 600), "budget");
}

fn protected(mut b: Value) -> Value {
    for d in b["data"].as_object_mut().unwrap().values_mut() {
        d["protection"]["participants"] = json!({"actor_reviewer":true});
    }
    b
}
fn protection_refuse(b: Value, code: &str) {
    let e = envelope(b);
    check_control(&e, &resources()).unwrap();
    let error = check_protection(&e, &resources()).unwrap_err();
    assert_eq!(error.code, code);
    assert!(e.document().pointer(&error.path).is_some(), "{error:?}");
}
fn secret_branch() -> Value {
    let mut b = branch();
    cell(&mut b, "secret", ROOT, false);
    b["data"]["secret"]["type"]["body"] = json!({"kind":"boolean"});
    b["data"]["secret"]["initial"] = "yes".into();
    b["data"]["secret"]["protection"]["purposes"] = json!({"private":true});
    expr(
        &mut b,
        "secret_guard",
        json!({"kind":"boolean"}),
        json!({"op":"read","cell":"secret"}),
        json!({"secret":true}),
    );
    b["nodes"]["choose"]["guards"]["left"] = "secret_guard".into();
    b["nodes"]["choose"]["reads"] = json!({"secret":true});
    protected(b)
}
#[test]
fn protection_rejects_implicit_branch_flows_and_keeps_compatible_labels() {
    let mut b = secret_branch();
    protection_refuse(b.clone(), "control-information-flow");
    b["data"]["answer"]["protection"]["purposes"] = json!({"private":true});
    check_protection(&envelope(b), &resources()).unwrap();
}
#[test]
fn protection_does_not_release_origins_after_merges_or_constant_guards() {
    let mut b = secret_branch();
    b["data"]["answer"]["protection"]["purposes"] = json!({"private":true});
    cell(&mut b, "public", ROOT, false);
    compute(
        &mut b,
        "after_merge",
        ROOT,
        json!({"public":"zero"}),
        json!({}),
    );
    for edge in ["left_done", "right_done"] {
        b["flows"][edge]["target"] = "after_merge".into();
    }
    flow(
        &mut b,
        "after_merge_done",
        "after_merge",
        "next",
        "node_done",
    );
    b["scopes"][ROOT]["outputs"] = json!({"result":"public"});
    protection_refuse(protected(b), "control-information-flow");
    let mut b = secret_branch();
    b["nodes"]["choose"]["guards"]["left"] = "yes".into();
    protection_refuse(b, "control-information-flow");
}
#[test]
fn protection_checks_initializers_and_invocation_input_transfer() {
    let mut b = base();
    cell(&mut b, "source", ROOT, false);
    cell(&mut b, "destination", ROOT, false);
    b["scopes"][ROOT]["inputs"] = json!({"value":"source"});
    read(&mut b, "read_source", "source");
    b["data"]["source"]["protection"]["purposes"] = json!({"private":true});
    b["data"]["destination"]["initial"] = "read_source".into();
    protection_refuse(protected(b), "control-information-flow");
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "source", ROOT, true);
    cell(&mut b, "local", "child", false);
    b["data"]["source"]["protection"]["purposes"] = json!({"private":true});
    b["scopes"]["child"]["inputs"] = json!({"value":"local"});
    read(&mut b, "argument", "source");
    invoke(&mut b, "call", "child", json!({}));
    b["nodes"]["call"]["inputs"] = json!({"value":"argument"});
    b["nodes"]["call"]["reads"] = json!({"source":true});
    protection_refuse(protected(b), "control-information-flow");
}
#[test]
fn protection_checks_input_destination_access_predicate_dependencies() {
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "secret", ROOT, true);
    cell(&mut b, "local", "child", false);
    b["data"]["secret"]["protection"]["purposes"] = json!({"private":true});
    b["policies"]["input_protection"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"protection","reads":{"secret":true},"permit":{"op":"eq","left":{"op":"read","cell":"secret"},"right":{"op":"literal","type":{"kind":"integer"},"value":"0"}}}});
    b["data"]["local"]["protection"]["policy"] = "input_protection".into();
    b["scopes"]["child"]["inputs"] = json!({"value":"local"});
    b["nodes"]["child_done"]["reads"] = json!({"local":true,"secret":true});
    invoke(&mut b, "call", "child", json!({}));
    b["nodes"]["call"]["inputs"] = json!({"value":"zero"});
    b["nodes"]["call"]["reads"] = json!({"secret":true});
    protection_refuse(protected(b), "control-information-flow");
}
#[test]
fn protection_keeps_nested_and_parallel_completion_origins() {
    let mut b = parallel();
    cell(&mut b, "public", ROOT, true);
    cell(&mut b, "secret", "child_a", true);
    b["data"]["secret"]["protection"]["purposes"] = json!({"private":true});
    b["nodes"]["child_a_done"]["reads"] = json!({"secret":true});
    b["scopes"][ROOT]["outputs"] = json!({"result":"public"});
    protection_refuse(protected(b), "control-information-flow");
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "secret", "child", true);
    cell(&mut b, "public", ROOT, false);
    b["data"]["secret"]["protection"]["purposes"] = json!({"private":true});
    b["nodes"]["child_done"]["reads"] = json!({"secret":true});
    invoke(&mut b, "call", "child", json!({}));
    compute(&mut b, "later", ROOT, json!({"public":"zero"}), json!({}));
    b["flows"]["flow_call"]["target"] = "later".into();
    flow(&mut b, "later_done", "later", "next", "node_done");
    protection_refuse(protected(b), "control-information-flow");
}
#[test]
fn protection_access_obligations_are_explicit_and_empty_contexts_deny() {
    protection_refuse(branch(), "access-context-empty");
    let b = protected(branch());
    let e = envelope(b);
    let report = check_protection(&e, &resources()).unwrap();
    assert_eq!(report.revision(), e.document()["revision"]);
    assert!(
        report
            .accesses()
            .iter()
            .any(|a| a.site() == "write_left" && a.cell() == "answer" && a.operation() == "write")
    );
    assert!(
        report.accesses().iter().any(|a| a.site() == "node_done"
            && a.cell() == "answer"
            && a.operation() == "transfer")
    );
    assert!(
        report
            .remaining()
            .contains(&"current-principal-purpose-capability-and-policy-enforcement")
    );
}
#[test]
fn protection_reports_unreachable_work_without_fabricated_access_contexts() {
    let mut b = base();
    cell(&mut b, "unused", ROOT, false);
    compute(
        &mut b,
        "unreachable",
        ROOT,
        json!({"unused":"zero"}),
        json!({}),
    );
    flow(&mut b, "unused_done", "unreachable", "next", "node_done");
    let report = check_protection(&envelope(b), &resources()).unwrap();
    assert!(report.control().uncovered_nodes().contains("unreachable"));
    assert!(report.accesses().is_empty());
}
#[test]
fn protection_graph_work_is_iterative_and_transfer_work_is_bounded() {
    check_protection(&envelope(base()), &resources()).unwrap();
    check_protection(&envelope(chain(1200, 0)), &resources()).unwrap();
    let mut b = chain(600, 0);
    for i in 0..200 {
        let id = format!("argument_{i:03}");
        cell(&mut b, &id, ROOT, false);
        b["scopes"][ROOT]["inputs"][format!("port_{i:03}")] = id.into();
    }
    let assignments: serde_json::Map<_, _> = (0..10)
        .map(|i| (format!("target_{i:03}"), json!("zero")))
        .collect();
    for id in assignments.keys() {
        cell(&mut b, id, ROOT, false);
    }
    for i in 0..600 {
        compute(
            &mut b,
            &format!("chain_{i:04}"),
            ROOT,
            Value::Object(assignments.clone()),
            json!({}),
        );
    }
    protection_refuse(protected(b), "budget");

    // Charge metadata comparisons too, rather than treating wide envelopes as
    // one unit per source/destination pair in the earlier static stage.
    let mut b = chain(200, 0);
    cell(&mut b, "source", ROOT, true);
    cell(&mut b, "destination", ROOT, false);
    read(&mut b, "copy_source", "source");
    let purposes: serde_json::Map<_, _> = (0..4000)
        .map(|i| (format!("purpose_{i:04}"), json!(true)))
        .collect();
    for id in ["source", "destination"] {
        b["data"][id]["protection"]["purposes"] = Value::Object(purposes.clone());
    }
    for i in 0..200 {
        compute(
            &mut b,
            &format!("chain_{i:04}"),
            ROOT,
            json!({"destination":"copy_source"}),
            json!({"source":true}),
        );
    }
    assert_eq!(
        check(&envelope(protected(b)), &resources())
            .unwrap_err()
            .code,
        "budget"
    );
}

fn access_policy(b: &mut Value, id: &str, source: &str) {
    b["policies"][id] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"protection","reads":{source:true},"permit":{"op":"eq","left":{"op":"read","cell":source},"right":{"op":"literal","type":{"kind":"integer"},"value":"0"}}}});
}
#[test]
fn initial_access_predicates_require_available_entry_data() {
    let mut b = base();
    cell(&mut b, "basis", ROOT, false);
    cell(&mut b, "value", ROOT, true);
    access_policy(&mut b, "access_basis", "basis");
    b["data"]["value"]["protection"]["policy"] = "access_basis".into();
    refuse(b.clone(), "initial-access-availability-unproved");
    b["scopes"][ROOT]["inputs"] = json!({"basis":"basis"});
    pass(b);
}
#[test]
fn initial_access_predicates_do_not_guess_local_or_peer_input_order() {
    let mut b = base();
    cell(&mut b, "basis", ROOT, true);
    cell(&mut b, "value", ROOT, true);
    access_policy(&mut b, "access_basis", "basis");
    b["data"]["value"]["protection"]["policy"] = "access_basis".into();
    refuse(b.clone(), "initial-access-availability-unproved");
    b["data"]["basis"]
        .as_object_mut()
        .unwrap()
        .remove("initial");
    b["data"]["value"]
        .as_object_mut()
        .unwrap()
        .remove("initial");
    b["scopes"][ROOT]["inputs"] = json!({"basis":"basis","value":"value"});
    refuse(b, "initial-access-availability-unproved");
}
#[test]
fn initial_access_dependency_expansion_handles_transitive_cycles_boundedly() {
    let mut b = base();
    cell(&mut b, "first", ROOT, true);
    cell(&mut b, "second", ROOT, true);
    access_policy(&mut b, "first_policy", "second");
    access_policy(&mut b, "second_policy", "first");
    b["data"]["first"]["protection"]["policy"] = "first_policy".into();
    b["data"]["second"]["protection"]["policy"] = "second_policy".into();
    refuse(b, "initial-access-availability-unproved");
}
#[test]
fn initial_access_includes_participant_requirement_dependencies() {
    let mut b = base();
    cell(&mut b, "basis", ROOT, false);
    cell(&mut b, "value", ROOT, true);
    b["data"]["value"]["protection"]["participants"] = json!({"actor_reviewer":true});
    b["policies"]["policy_actor"]["body"]["permit"] = json!({"op":"eq","left":{"op":"read","cell":"basis"},"right":{"op":"literal","type":{"kind":"integer"},"value":"0"}});
    b["policies"]["policy_actor"]["body"]["reads"] = json!({"basis":true});
    b["policies"]["policy_stop"]["body"]["reads"] = json!({"basis":true});
    b["nodes"]["node_done"]["reads"] = json!({"basis":true});
    refuse(b, "initial-access-availability-unproved");
}
#[test]
fn child_input_access_dependencies_use_caller_reads_and_ancestor_state() {
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "basis", ROOT, true);
    cell(&mut b, "local", "child", false);
    access_policy(&mut b, "access_basis", "basis");
    b["data"]["local"]["protection"]["policy"] = "access_basis".into();
    b["scopes"]["child"]["inputs"] = json!({"value":"local"});
    invoke(&mut b, "call", "child", json!({}));
    b["nodes"]["call"]["inputs"] = json!({"value":"zero"});
    b["nodes"]["call"]["reads"] = json!({"basis":true});
    pass(b.clone());
    let mut missing = b.clone();
    missing["nodes"]["call"]["reads"] = json!({});
    assert_eq!(
        check(&envelope(missing), &resources()).unwrap_err().code,
        "node-reads"
    );
    b["data"]["basis"]
        .as_object_mut()
        .unwrap()
        .remove("initial");
    refuse(b, "data-availability-unproved");
}
#[test]
fn fanout_item_access_metadata_is_a_parent_dependency_without_an_item_value_read() {
    let mut b = fanout();
    cell(&mut b, "basis", ROOT, true);
    access_policy(&mut b, "item_access", "basis");
    b["data"]["item_cell"]["protection"]["policy"] = "item_access".into();
    b["nodes"]["fork"]["reads"] = json!({"basis":true});
    b["nodes"]["child_a_done"]["reads"] = json!({"item_cell":true,"basis":true});
    pass(b.clone());
    b["nodes"]["fork"]["reads"] = json!({});
    assert_eq!(
        check(&envelope(b), &resources()).unwrap_err().code,
        "node-reads"
    );
}

#[test]
fn finish_declares_output_metadata_dependencies_in_uncalled_child_scopes() {
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "basis", ROOT, true);
    cell(&mut b, "local", "child", false);
    access_policy(&mut b, "output_access", "basis");
    b["data"]["local"]["protection"]["policy"] = "output_access".into();
    b["scopes"]["child"]["inputs"] = json!({"value":"local"});
    b["scopes"]["child"]["outputs"] = json!({"value":"local"});
    assert_eq!(
        check(&envelope(b.clone()), &resources()).unwrap_err().code,
        "node-reads"
    );
    b["nodes"]["child_done"]["reads"] = json!({"basis":true});
    let report = check_control(&envelope(b), &resources()).unwrap();
    assert!(report.uncovered_nodes().contains("child_done"));
}
#[test]
fn completion_metadata_closure_includes_access_on_predicate_read_data() {
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "audit", ROOT, false);
    cell(&mut b, "basis", ROOT, true);
    cell(&mut b, "local", "child", false);
    b["scopes"][ROOT]["inputs"] = json!({"audit":"audit"});
    access_policy(&mut b, "basis_access", "audit");
    access_policy(&mut b, "output_access", "basis");
    b["data"]["basis"]["protection"]["policy"] = "basis_access".into();
    b["data"]["local"]["protection"]["policy"] = "output_access".into();
    b["scopes"]["child"]["inputs"] = json!({"value":"local"});
    b["scopes"]["child"]["outputs"] = json!({"value":"local"});
    b["nodes"]["child_done"]["reads"] = json!({"basis":true});
    assert_eq!(
        check(&envelope(b.clone()), &resources()).unwrap_err().code,
        "node-reads"
    );
    b["nodes"]["child_done"]["reads"] = json!({"basis":true,"audit":true});
    check_control(&envelope(b), &resources()).unwrap();
}
#[test]
fn finish_metadata_has_no_spurious_output_value_reads() {
    let mut b = base();
    cell(&mut b, "value", ROOT, true);
    b["scopes"][ROOT]["outputs"] = json!({"value":"value"});
    assert_eq!(b["nodes"]["node_done"]["reads"], json!({}));
    check_control(&envelope(b), &resources()).unwrap();

    // Many terminal declarations share one wide, immutable envelope. Validate it
    // once, while still checking every node's metadata dependencies and budget.
    let mut b = base();
    cell(&mut b, "value", ROOT, true);
    b["scopes"][ROOT]["outputs"] = json!({"value":"value"});
    b["data"]["value"]["protection"]["purposes"] = Value::Object(
        (0..4000)
            .map(|i| (format!("purpose_{i:04}"), json!(true)))
            .collect(),
    );
    for i in 0..700 {
        b["nodes"][format!("extra_finish_{i:04}")] = b["nodes"]["node_done"].clone();
    }
    let report = check_control(&envelope(protected(b)), &resources()).unwrap();
    assert_eq!(report.uncovered_nodes().len(), 700);
}

#[test]
fn inherited_empty_structural_policies_remain_reusable() {
    let mut b = base();
    child(&mut b, "child");
    pass(b);
}
#[test]
fn fault_handlers_are_children_of_the_actual_policy_use_scope() {
    let mut b = base();
    child(&mut b, "handler");
    b["policies"]["empty_faults"] = b["policies"]["policy_faults"].clone();
    b["policies"]["policy_faults"]["body"]["handlers"] = json!({"recoverable":"handler"});
    b["scopes"]["handler"]["faults"] = "empty_faults".into();
    pass(b.clone());
    b["scopes"]["handler"]["faults"] = "policy_faults".into();
    let e = envelope(b);
    let error = check(&e, &resources()).unwrap_err();
    assert_eq!(error.code, "fault-handler-context");
    assert_eq!(error.path, "/body/scopes/handler/faults");
    assert!(e.document().pointer(&error.path).is_some());
}
#[test]
fn reconcile_followup_is_a_child_of_the_actual_policy_use_scope() {
    let mut b = base();
    child(&mut b, "followup");
    b["policies"]["terminal_close"] = b["policies"]["policy_close"].clone();
    b["policies"]["policy_close"]["body"]["mode"] = "reconcile".into();
    b["policies"]["policy_close"]["body"]["followup"] = "followup".into();
    b["scopes"]["followup"]["closure"] = "terminal_close".into();
    pass(b.clone());
    b["scopes"]["followup"]["closure"] = "policy_close".into();
    let e = envelope(b);
    let error = check(&e, &resources()).unwrap_err();
    assert_eq!(error.code, "closure-followup-context");
    assert_eq!(error.path, "/body/scopes/followup/closure");
    assert!(e.document().pointer(&error.path).is_some());
}
#[test]
fn ordinary_wait_policy_reuse_does_not_require_a_unique_non_node_owner() {
    let mut b = base();
    b["policies"]["observe_policy"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"wait","reads":{},"observationType":{"kind":"record","fields":{}},"match":{"op":"literal","type":{"kind":"boolean"},"value":true},"authority":"actor_reviewer","timer":null,"onObservation":"observed","onTimer":null}});
    for id in ["wait_a", "wait_b"] {
        b["nodes"][id] = json!({"kind":"wait","scope":ROOT,"policy":"observe_policy","reads":{},"writes":{},"outcomes":{"observed":true}});
    }
    flow(&mut b, "wait_a_next", "wait_a", "observed", "wait_b");
    flow(&mut b, "wait_b_next", "wait_b", "observed", "node_done");
    entry(&mut b, "wait_a");
    pass(b);
}

#[test]
fn all_creator_kinds_declare_ancestor_initializer_reads() {
    for kind in ["invoke", "repeat", "split", "fanout"] {
        let mut b = match kind {
            "split" => parallel(),
            "fanout" => fanout(),
            _ => {
                let mut b = base();
                child(&mut b, "child_a");
                b
            }
        };
        let creator = if matches!(kind, "split" | "fanout") {
            "fork"
        } else {
            "call"
        };
        if kind == "invoke" {
            invoke(&mut b, creator, "child_a", json!({}));
        } else if kind == "repeat" {
            b["nodes"][creator] = json!({"kind":"repeat","scope":ROOT,"body":"child_a","condition":"yes","reads":{},"writes":{},"outcomes":{"next":true}});
            entry(&mut b, creator);
            flow(&mut b, "repeat_next", creator, "next", "node_done");
        }
        cell(&mut b, "basis", ROOT, true);
        cell(&mut b, "local", "child_a", false);
        read(&mut b, "initial_read", "basis");
        b["data"]["local"]["initial"] = "initial_read".into();
        let e = envelope(b.clone());
        let error = check(&e, &resources()).unwrap_err();
        assert_eq!(error.code, "node-reads", "{kind}: {error:?}");
        assert_eq!(error.path, format!("/body/nodes/{creator}"));
        b["nodes"][creator]["reads"] = json!({"basis":true});
        pass(b.clone());
        b["data"]["basis"]
            .as_object_mut()
            .unwrap()
            .remove("initial");
        refuse(b, "data-availability-unproved");
    }
}

#[test]
fn creation_includes_initializer_source_access_metadata() {
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "audit", ROOT, false);
    b["scopes"][ROOT]["inputs"] = json!({"audit":"audit"});
    cell(&mut b, "basis", ROOT, true);
    cell(&mut b, "local", "child", false);
    access_policy(&mut b, "basis_access", "audit");
    b["data"]["basis"]["protection"]["policy"] = "basis_access".into();
    read(&mut b, "initial_read", "basis");
    b["data"]["local"]["initial"] = "initial_read".into();
    invoke(&mut b, "call", "child", json!({}));
    b["nodes"]["call"]["reads"] = json!({"basis":true,"audit":true});
    pass(b.clone());
    b["nodes"]["call"]["reads"] = json!({"basis":true});
    assert_eq!(
        check(&envelope(b), &resources()).unwrap_err().code,
        "node-reads"
    );
}

#[test]
fn creation_includes_transitive_initializer_destination_access_metadata() {
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "audit", ROOT, false);
    b["scopes"][ROOT]["inputs"] = json!({"audit":"audit"});
    cell(&mut b, "basis", ROOT, true);
    cell(&mut b, "local", "child", true);
    access_policy(&mut b, "basis_access", "audit");
    access_policy(&mut b, "local_access", "basis");
    b["data"]["basis"]["protection"]["policy"] = "basis_access".into();
    b["data"]["local"]["protection"]["policy"] = "local_access".into();
    invoke(&mut b, "call", "child", json!({}));
    b["nodes"]["call"]["reads"] = json!({"basis":true,"audit":true});
    pass(b.clone());
    // Still checked when the creator itself is ordinary-unreachable.
    entry(&mut b, "node_done");
    pass(b.clone());
    b["nodes"]["call"]["reads"] = json!({"basis":true});
    assert_eq!(
        check(&envelope(b), &resources()).unwrap_err().code,
        "node-reads"
    );
}

#[test]
fn fresh_input_and_initializer_cells_do_not_become_caller_value_reads() {
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "argument", "child", false);
    cell(&mut b, "local", "child", false);
    b["scopes"]["child"]["inputs"] = json!({"value":"argument"});
    read(&mut b, "copy_argument", "argument");
    b["expressions"]["copy_argument"]["scope"] = "child".into();
    access_policy(&mut b, "local_access", "argument");
    b["policies"]["local_access"]["scope"] = "child".into();
    b["data"]["local"]["protection"]["policy"] = "local_access".into();
    b["data"]["local"]["initial"] = "copy_argument".into();
    invoke(&mut b, "call", "child", json!({}));
    b["nodes"]["call"]["inputs"] = json!({"value":"zero"});
    assert_eq!(b["nodes"]["call"]["reads"], json!({}));
    pass(b);
}

fn invalidation_refuse(b: Value, code: &str) {
    let e = envelope(protected(b));
    check_protection(&e, &resources()).unwrap();
    let error = check_invalidation(&e, &resources()).unwrap_err();
    assert_eq!(error.code, code, "{error:?}");
    assert!(e.document().pointer(&error.path).is_some());
    assert!(!format!("{error:?}").contains(INTEGER_MARKER));
}

#[test]
fn invalidation_requires_direct_read_coverage_without_guessing_immutability() {
    let mut b = base();
    cell(&mut b, "basis", ROOT, true);
    compute(&mut b, "observe", ROOT, json!({}), json!({"basis":true}));
    entry(&mut b, "observe");
    flow(&mut b, "observe_done", "observe", "next", "node_done");
    invalidation_refuse(b.clone(), "invalidation-coverage");
    b["data"]["basis"]["invalidates"] = json!({"observe":true});
    let e = envelope(protected(b));
    let report = check_invalidation(&e, &resources()).unwrap();
    assert_eq!(report.revision(), e.document()["revision"]);
    assert_eq!(
        report
            .checked_nodes()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["observe"]
    );
    let obligation = report.obligations().first().unwrap();
    assert_eq!((obligation.site(), obligation.cell()), ("observe", "basis"));
    assert!(report.uncovered_nodes().is_empty());
}

#[test]
fn invalidation_retains_control_and_both_branch_origins_after_merge() {
    let mut b = branch();
    for (cell_id, node) in [("left_basis", "write_left"), ("right_basis", "write_right")] {
        cell(&mut b, cell_id, ROOT, true);
        b["nodes"][node]["reads"] = json!({cell_id:true});
        b["data"][cell_id]["invalidates"] = json!({"after_merge":true});
        let flow = if node == "write_left" {
            "left_done"
        } else {
            "right_done"
        };
        b["flows"][flow]["target"] = "after_merge".into();
    }
    b["data"]["answer"]["invalidates"] = json!({"after_merge":true});
    compute(&mut b, "after_merge", ROOT, json!({}), json!({}));
    flow(&mut b, "merge_done", "after_merge", "next", "node_done");
    let report = check_invalidation(&envelope(protected(b.clone())), &resources()).unwrap();
    assert_eq!(report.obligations().len(), 3);
    b["data"]["right_basis"]["invalidates"] = json!({});
    invalidation_refuse(b, "invalidation-coverage");
}

#[test]
fn invalidation_includes_work_policy_dependencies() {
    let mut b = base();
    cell(&mut b, "audit", ROOT, false);
    b["scopes"][ROOT]["inputs"] = json!({"audit":"audit"});
    cell(&mut b, "basis", ROOT, true);
    read(&mut b, "work_input", "basis");
    b["policies"]["work"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"work","reads":{"audit":true},"instructions":"Review evidence","authority":"actor_reviewer","retry":{"maxAttempts":1,"delays":[],"faults":{}},"timeout":null,"completion":{"op":"eq","left":{"op":"read","cell":"audit"},"right":{"op":"literal","type":{"kind":"integer"},"value":"0"}},"compensates":null}});
    b["nodes"]["observe"] = json!({"kind":"activity","scope":ROOT,"mode":"human","target":"actor_reviewer","policy":"work","input":"work_input","result":null,"reads":{"basis":true,"audit":true},"writes":{},"outcomes":{"next":true}});
    entry(&mut b, "observe");
    flow(&mut b, "observe_done", "observe", "next", "node_done");
    b["data"]["basis"]["invalidates"] = json!({"observe":true});
    invalidation_refuse(b.clone(), "invalidation-coverage");
    b["data"]["audit"]["invalidates"] = json!({"observe":true});
    check_invalidation(&envelope(protected(b)), &resources()).unwrap();
}

#[test]
fn invalidation_unions_repeated_child_contexts() {
    let mut b = base();
    child(&mut b, "child");
    for id in ["first_basis", "second_basis"] {
        cell(&mut b, id, ROOT, true);
        b["data"][id]["invalidates"] = json!({"child_observe":true});
    }
    compute(&mut b, "child_observe", "child", json!({}), json!({}));
    b["scopes"]["child"]["entry"] = "child_observe".into();
    flow(&mut b, "child_next", "child_observe", "next", "child_done");
    invoke(&mut b, "call_first", "child", json!({}));
    invoke(&mut b, "call_second", "child", json!({}));
    b["nodes"]["call_first"]["reads"] = json!({"first_basis":true});
    b["nodes"]["call_second"]["reads"] = json!({"second_basis":true});
    b["flows"]["flow_call_first"]["target"] = "call_second".into();
    entry(&mut b, "call_first");
    let report = check_invalidation(&envelope(protected(b.clone())), &resources()).unwrap();
    assert_eq!(report.obligations().len(), 2);
    b["data"]["second_basis"]["invalidates"] = json!({});
    invalidation_refuse(b, "invalidation-coverage");
}

#[test]
fn invalidation_does_not_invent_cross_scope_links_for_local_completion_origins() {
    let mut b = base();
    child(&mut b, "child");
    cell(&mut b, "local", "child", true);
    compute(
        &mut b,
        "child_observe",
        "child",
        json!({}),
        json!({"local":true}),
    );
    b["data"]["local"]["invalidates"] = json!({"child_observe":true});
    b["scopes"]["child"]["entry"] = "child_observe".into();
    flow(&mut b, "child_next", "child_observe", "next", "child_done");
    invoke(&mut b, "call", "child", json!({}));
    compute(&mut b, "after_child", ROOT, json!({}), json!({}));
    b["flows"]["flow_call"]["target"] = "after_child".into();
    flow(&mut b, "after_next", "after_child", "next", "node_done");
    invalidation_refuse(b, "invalidation-context-unproved");
}

#[test]
fn invalidation_reports_unvisited_sites_without_inventing_entry_contexts() {
    let mut b = base();
    cell(&mut b, "unavailable", ROOT, false);
    compute(
        &mut b,
        "unvisited",
        ROOT,
        json!({}),
        json!({"unavailable":true}),
    );
    flow(&mut b, "unused_next", "unvisited", "next", "node_done");
    let report = check_invalidation(&envelope(protected(b)), &resources()).unwrap();
    assert!(report.checked_nodes().is_empty());
    assert_eq!(
        report
            .uncovered_nodes()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["unvisited"]
    );
    assert!(report.obligations().is_empty());
}

#[test]
fn invalidation_keeps_a_consumed_old_result_even_when_the_site_replaces_it() {
    let mut b = base();
    cell(&mut b, "result", ROOT, true);
    b["data"]["result"]["type"]["body"] = json!({"kind":"record","fields":{}});
    expr(
        &mut b,
        "empty_result",
        json!({"kind":"record","fields":{}}),
        json!({"op":"literal","type":{"kind":"record","fields":{}},"value":{}}),
        json!({}),
    );
    b["data"]["result"]["initial"] = "empty_result".into();
    b["policies"]["work"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"work","reads":{},"instructions":"Review evidence","authority":"actor_reviewer","retry":{"maxAttempts":1,"delays":[],"faults":{}},"timeout":null,"completion":{"op":"literal","type":{"kind":"boolean"},"value":true},"compensates":null}});
    b["nodes"]["observe"] = json!({"kind":"activity","scope":ROOT,"mode":"human","target":"actor_reviewer","policy":"work","input":"zero","result":"result","reads":{"result":true},"writes":{"result":true},"outcomes":{"next":true}});
    entry(&mut b, "observe");
    flow(&mut b, "observe_done", "observe", "next", "node_done");
    invalidation_refuse(b.clone(), "invalidation-coverage");
    b["data"]["result"]["invalidates"] = json!({"observe":true});
    let report = check_invalidation(&envelope(protected(b)), &resources()).unwrap();
    assert_eq!(report.obligations().first().unwrap().cell(), "result");
}

fn concurrency_refuse(b: Value, code: &str) {
    let e = envelope(protected(b));
    check_invalidation(&e, &resources()).unwrap();
    let error = check_concurrency(&e, &resources()).unwrap_err();
    assert_eq!(error.code, code, "{error:?}");
    if code != "budget" {
        assert!(e.document().pointer(&error.path).is_some());
    }
    assert!(!format!("{error:?}").contains(INTEGER_MARKER));
}
fn child_compute(b: &mut Value, child: &str, assignments: Value, reads: Value) {
    let id = format!("{child}_work");
    compute(b, &id, child, assignments, reads);
    b["scopes"][child]["entry"] = id.clone().into();
    flow(
        b,
        &format!("{child}_next"),
        &id,
        "next",
        &format!("{child}_done"),
    );
}

#[test]
fn concurrency_treats_reused_template_and_fanout_locals_as_fresh_storage() {
    for fan in [false, true] {
        let mut b = if fan { fanout() } else { parallel() };
        if !fan {
            b["nodes"]["fork"]["children"]["b"] = "child_a".into();
        }
        cell(&mut b, "local", "child_a", false);
        child_compute(&mut b, "child_a", json!({"local":"zero"}), json!({}));
        let report = check_concurrency(&envelope(protected(b)), &resources()).unwrap();
        assert!(report.checked_forks().contains("fork"));
        assert!(report.uncovered_forks().is_empty());
    }
}

#[test]
fn concurrency_allows_shared_reads_and_disjoint_ancestor_writes() {
    let mut b = parallel();
    cell(&mut b, "basis", ROOT, true);
    b["data"]["basis"]["invalidates"] = json!({"child_a_work":true,"child_b_work":true});
    for child in ["child_a", "child_b"] {
        child_compute(&mut b, child, json!({}), json!({"basis":true}));
    }
    check_concurrency(&envelope(protected(b)), &resources()).unwrap();
    let mut b = parallel();
    for (child, target) in [("child_a", "left"), ("child_b", "right")] {
        cell(&mut b, target, ROOT, false);
        child_compute(&mut b, child, json!({target:"zero"}), json!({}));
    }
    check_concurrency(&envelope(protected(b)), &resources()).unwrap();
}

#[test]
fn concurrency_refuses_shared_hazards_even_with_an_arbitrary_true_guard() {
    for read_write in [false, true] {
        let mut b = parallel();
        cell(&mut b, "basis", ROOT, true);
        child_compute(&mut b, "child_a", json!({"basis":"zero"}), json!({}));
        if read_write {
            child_compute(&mut b, "child_b", json!({}), json!({"basis":true}));
            b["data"]["basis"]["invalidates"] = json!({"child_b_work":true});
        } else {
            child_compute(&mut b, "child_b", json!({"basis":"zero"}), json!({}));
        }
        concurrency_refuse(b.clone(), "parallel-storage-conflict-unproved");
        b["policies"]["policy_races"]["body"]["mode"] = "guarded".into();
        b["policies"]["policy_races"]["body"]["invariant"] =
            json!({"op":"literal","type":{"kind":"boolean"},"value":true});
        concurrency_refuse(b, "parallel-storage-conflict-unproved");
    }
}

#[test]
fn concurrency_includes_initializer_only_shared_reads() {
    let mut b = parallel();
    cell(&mut b, "basis", ROOT, true);
    cell(&mut b, "local", "child_a", false);
    read(&mut b, "seed_local", "basis");
    b["data"]["local"]["initial"] = "seed_local".into();
    b["nodes"]["fork"]["reads"] = json!({"basis":true});
    child_compute(&mut b, "child_b", json!({"basis":"zero"}), json!({}));
    concurrency_refuse(b, "parallel-storage-conflict-unproved");
}

#[test]
fn concurrency_promotes_nested_ancestor_writes() {
    let mut b = parallel();
    child(&mut b, "grandchild");
    b["scopes"]["grandchild"]["parent"] = "child_a".into();
    cell(&mut b, "basis", ROOT, true);
    child_compute(&mut b, "grandchild", json!({"basis":"zero"}), json!({}));
    child_compute(&mut b, "child_b", json!({"basis":"zero"}), json!({}));
    invoke(&mut b, "call_nested", "grandchild", json!({}));
    b["nodes"]["call_nested"]["scope"] = "child_a".into();
    b["scopes"]["child_a"]["entry"] = "call_nested".into();
    b["flows"]["flow_call_nested"]["target"] = "child_a_done".into();
    entry(&mut b, "fork");
    concurrency_refuse(b, "parallel-storage-conflict-unproved");
}

#[test]
fn concurrency_proves_all_member_completion_only_from_sufficient_predicates() {
    let all = json!({"op":"all","outcomes":{"complete":true}});
    let any = json!({"op":"any","outcomes":{"complete":true}});
    let threshold = json!({"op":"atLeast","count":2,"outcomes":{"complete":true}});
    for predicate in [
        all.clone(),
        threshold.clone(),
        json!({"op":"and","terms":{"a":all.clone(),"b":any.clone()}}),
        json!({"op":"or","terms":{"a":all.clone(),"b":threshold}}),
    ] {
        let mut b = parallel();
        b["nodes"]["barrier"]["predicate"] = predicate;
        check_concurrency(&envelope(protected(b)), &resources()).unwrap();
    }
    for predicate in [
        any.clone(),
        json!({"op":"atLeast","count":1,"outcomes":{"complete":true}}),
        json!({"op":"or","terms":{"a":all,"b":any.clone()}}),
    ] {
        let mut b = parallel();
        b["nodes"]["barrier"]["predicate"] = predicate;
        concurrency_refuse(b, "parallel-lifetime-unproved");
    }
    let mut b = parallel();
    b["nodes"]["fork"]["children"] = json!({"a":"child_a"});
    b["nodes"]["barrier"]["predicate"] = any.clone();
    check_concurrency(&envelope(protected(b)), &resources()).unwrap();
    let mut b = fanout();
    b["nodes"]["barrier"]["predicate"] = any;
    concurrency_refuse(b, "parallel-lifetime-unproved");
}

#[test]
fn concurrency_refuses_surviving_reconciliation_lifetimes_for_all_creators() {
    for kind in ["split", "fanout", "invoke", "repeat"] {
        let mut b = if kind == "fanout" {
            fanout()
        } else {
            parallel()
        };
        child(&mut b, "reconcile");
        b["scopes"]["reconcile"]["parent"] = "child_a".into();
        b["policies"]["child_close"] = json!({"scope":"child_a","dialect":"core","body":{"kind":"closure","reads":{},"mode":"reconcile","followup":"reconcile"}});
        b["scopes"]["child_a"]["closure"] = "child_close".into();
        if kind == "invoke" {
            invoke(&mut b, "call", "child_a", json!({}));
        }
        if kind == "repeat" {
            b["nodes"]["call"] = json!({"kind":"repeat","scope":ROOT,"body":"child_a","condition":"yes","reads":{},"writes":{},"outcomes":{"next":true}});
            entry(&mut b, "call");
            flow(&mut b, "repeat_next", "call", "next", "node_done");
        }
        concurrency_refuse(b, "scope-lifetime-unproved");
    }
}

#[test]
fn concurrency_refuses_overlapping_opaque_external_work() {
    let mut b = parallel();
    b["policies"]["work"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"work","reads":{},"instructions":"Review evidence","authority":"actor_reviewer","retry":{"maxAttempts":1,"delays":[],"faults":{}},"timeout":null,"completion":{"op":"literal","type":{"kind":"boolean"},"value":true},"compensates":null}});
    for child in ["child_a", "child_b"] {
        let id = format!("{child}_work");
        b["nodes"][&id] = json!({"kind":"activity","scope":child,"mode":"human","target":"actor_reviewer","policy":"work","input":"zero","result":null,"reads":{},"writes":{},"outcomes":{"next":true}});
        b["scopes"][child]["entry"] = id.clone().into();
        flow(
            &mut b,
            &format!("{child}_next"),
            &id,
            "next",
            &format!("{child}_done"),
        );
    }
    concurrency_refuse(b.clone(), "parallel-effects-unproved");
    b["policies"]["observe_policy"] = json!({"scope":ROOT,"dialect":"core","body":{"kind":"wait","reads":{},"observationType":{"kind":"record","fields":{}},"match":{"op":"literal","type":{"kind":"boolean"},"value":true},"authority":"actor_reviewer","timer":null,"onObservation":"next","onTimer":null}});
    b["nodes"]["child_b_work"] = json!({"kind":"wait","scope":"child_b","policy":"observe_policy","reads":{},"writes":{},"outcomes":{"next":true}});
    concurrency_refuse(b.clone(), "parallel-effects-unproved");
    b["scopes"]["child_b"]["entry"] = "child_b_done".into();
    // Unvisited declarations remain in conservative summaries; deletion is an
    // explicit definition edit, not a constant-guard or activation guess.
    b["nodes"].as_object_mut().unwrap().remove("child_b_work");
    b["flows"].as_object_mut().unwrap().remove("child_b_next");
    check_concurrency(&envelope(protected(b)), &resources()).unwrap();
}

#[test]
fn concurrency_reports_unvisited_forks_and_bounds_member_work() {
    let mut b = parallel();
    entry(&mut b, "node_done");
    let report = check_concurrency(&envelope(protected(b)), &resources()).unwrap();
    assert!(report.checked_forks().is_empty());
    assert!(report.uncovered_forks().contains("fork"));
    let mut b = parallel();
    b["nodes"]["fork"]["children"] = (0..2200)
        .map(|i| (format!("branch_{i:04}"), json!("child_a")))
        .collect::<serde_json::Map<_, _>>()
        .into();
    // Empty peers use an accumulated index, not quadratic pair scans.
    check_concurrency(&envelope(protected(b.clone())), &resources()).unwrap();
    for i in 0..1000 {
        cell(&mut b, &format!("basis_{i:04}"), ROOT, false);
    }
    let reads: serde_json::Map<_, _> = (0..1000)
        .map(|i| (format!("basis_{i:04}"), true.into()))
        .collect();
    compute(
        &mut b,
        "unvisited_reads",
        "child_a",
        json!({}),
        reads.into(),
    );
    flow(
        &mut b,
        "unvisited_next",
        "unvisited_reads",
        "next",
        "child_a_done",
    );
    b["nodes"]["fork"]["children"] = (0..600)
        .map(|i| (format!("branch_{i:04}"), json!("child_a")))
        .collect::<serde_json::Map<_, _>>()
        .into();
    concurrency_refuse(b, "budget");
}

#[test]
fn concurrency_retains_pre_fork_control_reads_in_child_footprints() {
    let mut b = parallel();
    cell(&mut b, "basis", ROOT, true);
    cell(&mut b, "other", ROOT, true);
    expr(
        &mut b,
        "route_guard",
        json!({"kind":"boolean"}),
        json!({"op":"eq","left":{"op":"read","cell":"basis"},"right":{"op":"literal","type":{"kind":"integer"},"value":"0"}}),
        json!({"basis":true}),
    );
    b["nodes"]["route"] = json!({"kind":"decision","scope":ROOT,"guards":{"next":"route_guard"},"default":null,"reads":{"basis":true},"writes":{},"outcomes":{"next":true}});
    b["data"]["basis"]["invalidates"] = json!({"route":true});
    entry(&mut b, "route");
    flow(&mut b, "route_fork", "route", "next", "fork");
    child_compute(&mut b, "child_a", json!({"basis":"zero"}), json!({}));
    child_compute(&mut b, "child_b", json!({"other":"zero"}), json!({}));
    concurrency_refuse(b, "parallel-storage-conflict-unproved");
}

#[test]
fn concurrency_refuses_nonvisible_completion_context_without_aliasing_fresh_locals() {
    let mut b = parallel();
    child(&mut b, "seed");
    cell(&mut b, "seed_local", "seed", true);
    b["nodes"]["seed_done"]["reads"] = json!({"seed_local":true});
    invoke(&mut b, "seed_call", "seed", json!({}));
    b["flows"]["flow_seed_call"]["target"] = "fork".into();
    concurrency_refuse(b, "scope-context-unproved");
}

#[test]
fn concurrency_refuses_potential_shared_writes_between_fanout_occurrences() {
    let mut b = fanout();
    cell(&mut b, "shared", ROOT, false);
    child_compute(&mut b, "child_a", json!({"shared":"zero"}), json!({}));
    concurrency_refuse(b, "parallel-storage-conflict-unproved");
}
