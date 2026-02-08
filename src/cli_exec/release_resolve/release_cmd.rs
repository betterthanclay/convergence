use super::*;

pub(super) fn handle_release_create(
    client: &RemoteClient,
    channel: String,
    bundle_id: String,
    notes: Option<String>,
    json: bool,
) -> Result<()> {
    let r = client.create_release(&channel, &bundle_id, notes)?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&r).context("serialize release create json")?
        );
    } else {
        println!("{} {}", r.channel, r.bundle_id);
    }
    Ok(())
}

pub(super) fn handle_release_list(client: &RemoteClient, json: bool) -> Result<()> {
    let mut rs = client.list_releases()?;
    rs.sort_by(|a, b| b.released_at.cmp(&a.released_at));
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&rs).context("serialize release list json")?
        );
    } else {
        for r in rs {
            let short = r.bundle_id.chars().take(8).collect::<String>();
            println!(
                "{} {} {} {}",
                r.channel, short, r.released_at, r.released_by
            );
        }
    }
    Ok(())
}

pub(super) fn handle_release_show(
    client: &RemoteClient,
    channel: String,
    json: bool,
) -> Result<()> {
    let r = client.get_release(&channel)?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&r).context("serialize release show json")?
        );
    } else {
        println!("channel: {}", r.channel);
        println!("bundle: {}", r.bundle_id);
        println!("scope: {}", r.scope);
        println!("gate: {}", r.gate);
        println!("released_at: {}", r.released_at);
        println!("released_by: {}", r.released_by);
        if let Some(n) = r.notes {
            println!("notes: {}", n);
        }
    }
    Ok(())
}
