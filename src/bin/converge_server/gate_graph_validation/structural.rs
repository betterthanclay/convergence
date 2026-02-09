use super::*;

pub(super) fn run(graph: &GateGraph, issues: &mut Vec<GateGraphIssue>) -> Result<(), ()> {
    if graph.version != 1 {
        issues.push(GateGraphIssue {
            code: "unsupported_version".to_string(),
            message: "unsupported gate graph version".to_string(),
            gate: None,
            upstream: None,
        });
        return Err(());
    }

    if graph.gates.is_empty() {
        issues.push(GateGraphIssue {
            code: "no_gates".to_string(),
            message: "gate graph must contain at least one gate".to_string(),
            gate: None,
            upstream: None,
        });
        return Err(());
    }

    let mut ids = HashSet::new();
    for g in &graph.gates {
        if let Err(err) = validate_gate_id(&g.id) {
            issues.push(GateGraphIssue {
                code: "invalid_gate_id".to_string(),
                message: err.to_string(),
                gate: Some(g.id.clone()),
                upstream: None,
            });
        }
        if g.name.trim().is_empty() {
            issues.push(GateGraphIssue {
                code: "empty_gate_name".to_string(),
                message: "gate name cannot be empty".to_string(),
                gate: Some(g.id.clone()),
                upstream: None,
            });
        }
        if !ids.insert(g.id.clone()) {
            issues.push(GateGraphIssue {
                code: "duplicate_gate_id".to_string(),
                message: format!("duplicate gate id {}", g.id),
                gate: Some(g.id.clone()),
                upstream: None,
            });
        }
    }

    for g in &graph.gates {
        for up in &g.upstream {
            if let Err(err) = validate_gate_id(up) {
                issues.push(GateGraphIssue {
                    code: "invalid_upstream_id".to_string(),
                    message: err.to_string(),
                    gate: Some(g.id.clone()),
                    upstream: Some(up.clone()),
                });
                continue;
            }
            if !ids.contains(up) {
                issues.push(GateGraphIssue {
                    code: "unknown_upstream".to_string(),
                    message: format!("gate {} references unknown upstream {}", g.id, up),
                    gate: Some(g.id.clone()),
                    upstream: Some(up.clone()),
                });
            }
        }
    }

    Ok(())
}
