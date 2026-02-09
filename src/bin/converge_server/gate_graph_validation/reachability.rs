use super::*;

pub(super) fn run(graph: &GateGraph, issues: &mut Vec<GateGraphIssue>) {
    let roots: Vec<&GateDef> = graph
        .gates
        .iter()
        .filter(|g| g.upstream.is_empty())
        .collect();

    if roots.is_empty() {
        issues.push(GateGraphIssue {
            code: "no_root_gate".to_string(),
            message: "gate graph must contain at least one root gate (a gate with no upstream)"
                .to_string(),
            gate: None,
            upstream: None,
        });
        return;
    }

    let mut by_id: HashMap<String, &GateDef> = HashMap::new();
    for g in &graph.gates {
        by_id.insert(g.id.clone(), g);
    }

    let mut downstream: HashMap<String, Vec<String>> = HashMap::new();
    for g in &graph.gates {
        for up in &g.upstream {
            downstream.entry(up.clone()).or_default().push(g.id.clone());
        }
    }

    let mut stack: Vec<String> = roots.iter().map(|g| g.id.clone()).collect();
    let mut reachable: HashSet<String> = HashSet::new();
    while let Some(id) = stack.pop() {
        if !reachable.insert(id.clone()) {
            continue;
        }
        if let Some(next) = downstream.get(&id) {
            for nid in next {
                if by_id.contains_key(nid) {
                    stack.push(nid.clone());
                }
            }
        }
    }

    if reachable.len() != graph.gates.len() {
        let mut missing: Vec<String> = graph
            .gates
            .iter()
            .map(|g| g.id.clone())
            .filter(|id| !reachable.contains(id))
            .collect();
        missing.sort();
        issues.push(GateGraphIssue {
            code: "unreachable_gates".to_string(),
            message: format!(
                "unreachable gates (not reachable from any root): {}",
                missing.join(", ")
            ),
            gate: None,
            upstream: None,
        });
    }
}
