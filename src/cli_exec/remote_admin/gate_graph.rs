use super::super::*;

pub(super) fn handle_gates_command(ws: &Workspace, command: GateGraphCommands) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote.clone(), token)?;

    match command {
        GateGraphCommands::Show { json } => {
            let graph = client.get_gate_graph()?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&graph).context("serialize gate graph json")?
                );
            } else {
                let mut gates = graph.gates;
                gates.sort_by(|a, b| a.id.cmp(&b.id));
                for g in gates {
                    let ups = if g.upstream.is_empty() {
                        "(root)".to_string()
                    } else {
                        format!("<- {}", g.upstream.join(", "))
                    };
                    let release = if g.allow_releases { "" } else { " no-releases" };
                    println!("{} {}{}", g.id, ups, release);
                }
            }
        }
        GateGraphCommands::Set { file, json } => {
            let raw = std::fs::read_to_string(&file)
                .with_context(|| format!("read {}", file.display()))?;
            let graph: converge::remote::GateGraph =
                serde_json::from_str(&raw).context("parse gate graph json")?;
            let updated = client.put_gate_graph(&graph)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&updated).context("serialize gate graph json")?
                );
            } else {
                println!("updated gate graph");
            }
        }
        GateGraphCommands::Init { apply, json } => {
            let graph = converge::remote::GateGraph {
                version: 1,
                gates: vec![
                    converge::remote::GateDef {
                        id: "dev-intake".to_string(),
                        name: "Dev Intake".to_string(),
                        upstream: Vec::new(),
                        allow_releases: true,
                        allow_superpositions: false,
                        allow_metadata_only_publications: false,
                        required_approvals: 0,
                    },
                    converge::remote::GateDef {
                        id: "integrate".to_string(),
                        name: "Integrate".to_string(),
                        upstream: vec!["dev-intake".to_string()],
                        allow_releases: true,
                        allow_superpositions: false,
                        allow_metadata_only_publications: false,
                        required_approvals: 0,
                    },
                    converge::remote::GateDef {
                        id: "ship".to_string(),
                        name: "Ship".to_string(),
                        upstream: vec!["integrate".to_string()],
                        allow_releases: true,
                        allow_superpositions: false,
                        allow_metadata_only_publications: false,
                        required_approvals: 0,
                    },
                ],
            };

            if apply {
                let updated = client.put_gate_graph(&graph)?;
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&updated)
                            .context("serialize gate graph json")?
                    );
                } else {
                    let _ = updated;
                    println!("applied starter gate graph");
                }
            } else if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&graph).context("serialize gate graph json")?
                );
            } else {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&graph).context("serialize gate graph json")?
                );
                println!("hint: save to a file and run `converge gates set --file <path>`");
            }
        }
    }

    Ok(())
}
