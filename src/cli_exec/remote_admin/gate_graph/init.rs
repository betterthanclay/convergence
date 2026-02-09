use super::*;

pub(super) fn init_gate_graph(client: &RemoteClient, apply: bool, json: bool) -> Result<()> {
    let graph = starter_gate_graph();

    if apply {
        let updated = client.put_gate_graph(&graph)?;
        if json {
            println!(
                "{}",
                serde_json::to_string_pretty(&updated).context("serialize gate graph json")?
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

    Ok(())
}

fn starter_gate_graph() -> converge::remote::GateGraph {
    converge::remote::GateGraph {
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
    }
}
