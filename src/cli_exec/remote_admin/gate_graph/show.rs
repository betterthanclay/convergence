use super::*;

pub(super) fn show_gate_graph(client: &RemoteClient, json: bool) -> Result<()> {
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

    Ok(())
}
