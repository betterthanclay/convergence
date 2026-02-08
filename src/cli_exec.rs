use anyhow::{Context, Result};

use converge::remote::RemoteClient;
use converge::workspace::Workspace;

use crate::{
    GateGraphCommands, LaneCommands, LaneMembersCommands, MembersCommands, RemoteCommands,
    TokenCommands, UserCommands, require_remote_and_token,
};

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

pub(super) fn handle_token_command(ws: &Workspace, command: TokenCommands) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote, token)?;

    match command {
        TokenCommands::Create { label, user, json } => {
            let created = if let Some(handle) = user.as_deref() {
                let users = client.list_users()?;
                let uid = users
                    .iter()
                    .find(|u| u.handle == handle)
                    .map(|u| u.id.clone())
                    .with_context(|| format!("unknown user handle: {}", handle))?;
                client.create_token_for_user(&uid, label)?
            } else {
                client.create_token(label)?
            };
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&created)
                        .context("serialize token create json")?
                );
            } else {
                println!("token_id: {}", created.id);
                println!("token: {}", created.token);
                println!("created_at: {}", created.created_at);
                println!("note: token is shown once; store it now");
            }
        }
        TokenCommands::List { json } => {
            let list = client.list_tokens()?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&list).context("serialize token list json")?
                );
            } else {
                for t in list {
                    let label = t.label.unwrap_or_default();
                    let revoked = if t.revoked_at.is_some() {
                        " revoked"
                    } else {
                        ""
                    };
                    println!("{} {}{}", t.id, label, revoked);
                }
            }
        }
        TokenCommands::Revoke { id, json } => {
            client.revoke_token(&id)?;
            if json {
                println!("{}", serde_json::json!({"revoked": true, "id": id}));
            } else {
                println!("Revoked {}", id);
            }
        }
    }

    Ok(())
}

pub(super) fn handle_user_command(ws: &Workspace, command: UserCommands) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote, token)?;

    match command {
        UserCommands::List { json } => {
            let mut users = client.list_users()?;
            users.sort_by(|a, b| a.handle.cmp(&b.handle));
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&users).context("serialize users json")?
                );
            } else {
                for u in users {
                    let admin = if u.admin { " admin" } else { "" };
                    println!("{} {}{}", u.id, u.handle, admin);
                }
            }
        }
        UserCommands::Create {
            handle,
            display_name,
            admin,
            json,
        } => {
            let created = client.create_user(&handle, display_name, admin)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&created).context("serialize user create json")?
                );
            } else {
                println!("user_id: {}", created.id);
                println!("handle: {}", created.handle);
                println!("admin: {}", created.admin);
            }
        }
    }

    Ok(())
}

pub(super) fn handle_members_command(ws: &Workspace, command: MembersCommands) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote, token)?;

    match command {
        MembersCommands::List { json } => {
            let m = client.list_repo_members()?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&m).context("serialize members json")?
                );
            } else {
                println!("owner: {}", m.owner);
                let publishers: std::collections::HashSet<String> =
                    m.publishers.into_iter().collect();
                let mut readers = m.readers;
                readers.sort();
                for r in readers {
                    let role = if publishers.contains(&r) {
                        "publish"
                    } else {
                        "read"
                    };
                    println!("{} {}", r, role);
                }
            }
        }
        MembersCommands::Add { handle, role, json } => {
            client.add_repo_member(&handle, &role)?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({"ok": true, "handle": handle, "role": role})
                );
            } else {
                println!("Added {} ({})", handle, role);
            }
        }
        MembersCommands::Remove { handle, json } => {
            client.remove_repo_member(&handle)?;
            if json {
                println!("{}", serde_json::json!({"ok": true, "handle": handle}));
            } else {
                println!("Removed {}", handle);
            }
        }
    }

    Ok(())
}

pub(super) fn handle_lane_command(ws: &Workspace, command: LaneCommands) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote, token)?;

    match command {
        LaneCommands::Members { lane_id, command } => match command {
            LaneMembersCommands::List { json } => {
                let m = client.list_lane_members(&lane_id)?;
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&m).context("serialize lane members json")?
                    );
                } else {
                    println!("lane: {}", m.lane);
                    let mut members = m.members;
                    members.sort();
                    for h in members {
                        println!("{}", h);
                    }
                }
            }
            LaneMembersCommands::Add { handle, json } => {
                client.add_lane_member(&lane_id, &handle)?;
                if json {
                    println!(
                        "{}",
                        serde_json::json!({"ok": true, "lane": lane_id, "handle": handle})
                    );
                } else {
                    println!("Added {} to lane {}", handle, lane_id);
                }
            }
            LaneMembersCommands::Remove { handle, json } => {
                client.remove_lane_member(&lane_id, &handle)?;
                if json {
                    println!(
                        "{}",
                        serde_json::json!({"ok": true, "lane": lane_id, "handle": handle})
                    );
                } else {
                    println!("Removed {} from lane {}", handle, lane_id);
                }
            }
        },
    }

    Ok(())
}
