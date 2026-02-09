use super::super::*;

pub(super) fn handle_remote_command(ws: &Workspace, command: RemoteCommands) -> Result<()> {
    match command {
        RemoteCommands::Show { json } => {
            let cfg = ws.store.read_config()?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&cfg.remote).context("serialize remote json")?
                );
            } else if let Some(remote) = cfg.remote {
                println!("url: {}", remote.base_url);
                println!("repo: {}", remote.repo_id);
                println!("scope: {}", remote.scope);
                println!("gate: {}", remote.gate);
            } else {
                println!("No remote configured");
            }
        }
        RemoteCommands::Set {
            url,
            token,
            repo,
            scope,
            gate,
        } => {
            let mut cfg = ws.store.read_config()?;
            let remote = converge::model::RemoteConfig {
                base_url: url,
                token: None,
                repo_id: repo,
                scope,
                gate,
            };
            ws.store
                .set_remote_token(&remote, &token)
                .context("store remote token in state.json")?;
            cfg.remote = Some(remote);
            ws.store.write_config(&cfg)?;
            println!("Remote configured");
        }
        RemoteCommands::CreateRepo { repo, json } => {
            let (remote, token) = require_remote_and_token(&ws.store)?;
            let repo_id = repo.unwrap_or_else(|| remote.repo_id.clone());
            let client = RemoteClient::new(remote, token)?;
            let created = client.create_repo(&repo_id)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&created).context("serialize repo create json")?
                );
            } else {
                println!("Created repo {}", created.id);
            }
        }

        RemoteCommands::Purge {
            dry_run,
            prune_metadata,
            prune_releases_keep_last,
            json,
        } => {
            let (remote, token) = require_remote_and_token(&ws.store)?;
            let client = RemoteClient::new(remote, token)?;
            let report = client.gc_repo(dry_run, prune_metadata, prune_releases_keep_last)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).context("serialize purge json")?
                );
            } else {
                let kept = report.get("kept").and_then(|v| v.as_object());
                let deleted = report.get("deleted").and_then(|v| v.as_object());
                println!("dry_run: {}", dry_run);
                println!("prune_metadata: {}", prune_metadata);
                if let Some(n) = prune_releases_keep_last {
                    println!("prune_releases_keep_last: {}", n);
                }
                if let Some(k) = kept {
                    println!(
                        "kept: bundles={} releases={} snaps={} blobs={} manifests={} recipes={}",
                        k.get("bundles").and_then(|v| v.as_u64()).unwrap_or(0),
                        k.get("releases").and_then(|v| v.as_u64()).unwrap_or(0),
                        k.get("snaps").and_then(|v| v.as_u64()).unwrap_or(0),
                        k.get("blobs").and_then(|v| v.as_u64()).unwrap_or(0),
                        k.get("manifests").and_then(|v| v.as_u64()).unwrap_or(0),
                        k.get("recipes").and_then(|v| v.as_u64()).unwrap_or(0),
                    );
                }
                if let Some(d) = deleted {
                    println!(
                        "deleted: bundles={} releases={} snaps={} blobs={} manifests={} recipes={}",
                        d.get("bundles").and_then(|v| v.as_u64()).unwrap_or(0),
                        d.get("releases").and_then(|v| v.as_u64()).unwrap_or(0),
                        d.get("snaps").and_then(|v| v.as_u64()).unwrap_or(0),
                        d.get("blobs").and_then(|v| v.as_u64()).unwrap_or(0),
                        d.get("manifests").and_then(|v| v.as_u64()).unwrap_or(0),
                        d.get("recipes").and_then(|v| v.as_u64()).unwrap_or(0),
                    );
                }
            }
        }
    }

    Ok(())
}
