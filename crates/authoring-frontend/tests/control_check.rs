// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use choreoform_authoring_frontend::Package;
use choreoform_ir_probe_core::{DefinitionEnvelope, Resource};
use choreoform_semantic_check::{CORE, SEMANTICS, check, check_control, check_protection};
use serde_json::{Value, json};

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
