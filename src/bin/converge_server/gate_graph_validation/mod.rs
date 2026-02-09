//! Gate graph structural validation (ID checks, cycles, and reachability).

use super::*;

mod cycles;
mod reachability;
mod structural;

#[derive(Clone, Debug, serde::Serialize)]
pub(super) struct GateGraphIssue {
    code: String,
    message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    gate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    upstream: Option<String>,
}

pub(super) fn validate_gate_graph_issues(graph: &GateGraph) -> Vec<GateGraphIssue> {
    let mut issues: Vec<GateGraphIssue> = Vec::new();

    if structural::run(graph, &mut issues).is_err() {
        return issues;
    }

    if cycles::run(graph, &mut issues).is_err() {
        return issues;
    }

    reachability::run(graph, &mut issues);
    issues
}
