use super::*;

pub(super) fn set_gate_graph(
    client: &RemoteClient,
    file: std::path::PathBuf,
    json: bool,
) -> Result<()> {
    let raw = std::fs::read_to_string(&file).with_context(|| format!("read {}", file.display()))?;
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
    Ok(())
}
