use super::*;

pub(super) fn format_gate_graph_validation_error(v: &GateGraphValidationError) -> String {
    if v.issues.is_empty() {
        return v.error.clone();
    }

    let mut lines: Vec<String> = Vec::new();
    lines.push(v.error.clone());
    for i in v.issues.iter().take(8) {
        let mut bits = Vec::new();
        bits.push(i.code.clone());
        if let Some(g) = &i.gate {
            bits.push(format!("gate={}", g));
        }
        if let Some(u) = &i.upstream {
            bits.push(format!("upstream={}", u));
        }
        lines.push(format!("- {}: {}", bits.join(" "), i.message));
    }
    if v.issues.len() > 8 {
        lines.push(format!("... and {} more", v.issues.len() - 8));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_validation_error_without_issues_returns_top_level_error() {
        let v = GateGraphValidationError {
            error: "invalid graph".to_string(),
            issues: Vec::new(),
        };
        assert_eq!(format_gate_graph_validation_error(&v), "invalid graph");
    }

    #[test]
    fn format_validation_error_limits_issue_lines() {
        let issues = (0..10)
            .map(|i| {
                serde_json::json!({
                    "code": "cycle",
                    "message": format!("issue {}", i),
                    "gate": format!("g{}", i),
                    "upstream": serde_json::Value::Null
                })
            })
            .collect::<Vec<_>>();
        let v: GateGraphValidationError = serde_json::from_value(serde_json::json!({
            "error": "invalid graph",
            "issues": issues
        }))
        .expect("parse validation error");
        let text = format_gate_graph_validation_error(&v);
        assert!(text.contains("invalid graph"));
        assert!(text.contains("- cycle gate=g0: issue 0"));
        assert!(text.contains("- cycle gate=g7: issue 7"));
        assert!(!text.contains("issue 8"));
        assert!(text.contains("... and 2 more"));
    }
}
