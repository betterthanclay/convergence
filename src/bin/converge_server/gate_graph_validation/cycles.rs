use super::*;

pub(super) fn run(graph: &GateGraph, issues: &mut Vec<GateGraphIssue>) -> Result<(), ()> {
    if issues.iter().any(|i| i.code == "unknown_upstream") {
        return Err(());
    }

    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    for g in &graph.gates {
        if let Err(err) = dfs_gate(g, graph, &mut visiting, &mut visited) {
            issues.push(GateGraphIssue {
                code: "cycle".to_string(),
                message: err.to_string(),
                gate: None,
                upstream: None,
            });
            return Err(());
        }
    }
    Ok(())
}

fn dfs_gate(
    gate: &GateDef,
    graph: &GateGraph,
    visiting: &mut HashSet<String>,
    visited: &mut HashSet<String>,
) -> Result<()> {
    if visited.contains(&gate.id) {
        return Ok(());
    }
    if !visiting.insert(gate.id.clone()) {
        return Err(anyhow::anyhow!("cycle detected at gate {}", gate.id));
    }

    for up in &gate.upstream {
        let up_gate = graph
            .gates
            .iter()
            .find(|g| g.id == *up)
            .ok_or_else(|| anyhow::anyhow!("unknown upstream {}", up))?;
        dfs_gate(up_gate, graph, visiting, visited)?;
    }

    visiting.remove(&gate.id);
    visited.insert(gate.id.clone());
    Ok(())
}
