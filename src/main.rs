use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use converge::workspace::Workspace;
use converge::{model::RemoteConfig, remote::RemoteClient, store::LocalStore};

mod cli_exec;

#[derive(Parser)]
#[command(name = "converge")]
#[command(about = "Convergence version control", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a workspace (.converge)
    Init {
        /// Re-initialize if .converge already exists
        #[arg(long)]
        force: bool,
        /// Path to initialize (defaults to current directory)
        #[arg(long)]
        path: Option<PathBuf>,
    },

    /// Create a snapshot of the current workspace state
    Snap {
        /// Optional snap message
        #[arg(short = 'm', long)]
        message: Option<String>,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// List snaps
    Snaps {
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Show a snap
    Show {
        snap_id: String,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Restore a snap into the working directory
    Restore {
        snap_id: String,
        /// Remove existing files before restoring
        #[arg(long)]
        force: bool,
    },

    /// Compute a basic diff (workspace vs HEAD, or snap vs snap)
    Diff {
        /// Base snap id
        #[arg(long)]
        from: Option<String>,
        /// Target snap id
        #[arg(long)]
        to: Option<String>,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Move/rename a file or directory within the workspace
    #[command(name = "mv")]
    Mv { from: String, to: String },

    /// Configure or show the remote
    Remote {
        #[command(subcommand)]
        command: RemoteCommands,
    },

    /// Manage a repo's gate graph (admin)
    #[command(name = "gates", alias = "gate-graph")]
    Gates {
        #[command(subcommand)]
        command: GateGraphCommands,
    },

    /// Log in to a remote (configure remote + store token)
    Login {
        #[arg(long)]
        url: String,
        #[arg(long)]
        token: String,
        #[arg(long)]
        repo: String,
        #[arg(long, default_value = "main")]
        scope: String,
        #[arg(long, default_value = "dev-intake")]
        gate: String,
    },

    /// Log out (clear stored remote token)
    Logout,

    /// Show current remote identity
    Whoami {
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Manage remote access tokens
    Token {
        #[command(subcommand)]
        command: TokenCommands,
    },

    /// Manage users (admin)
    User {
        #[command(subcommand)]
        command: UserCommands,
    },

    /// Publish a snap to the configured remote
    Publish {
        /// Snap id to publish (defaults to latest)
        #[arg(long)]
        snap_id: Option<String>,
        /// Override scope (defaults to remote config)
        #[arg(long)]
        scope: Option<String>,
        /// Override gate (defaults to remote config)
        #[arg(long)]
        gate: Option<String>,
        /// Create a metadata-only publication (skip uploading blobs)
        #[arg(long)]
        metadata_only: bool,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Sync a snap to your lane head (unpublished collaboration)
    Sync {
        /// Snap id to sync (defaults to latest)
        #[arg(long)]
        snap_id: Option<String>,
        /// Lane id (defaults to "default")
        #[arg(long, default_value = "default")]
        lane: String,
        /// Optional client identifier
        #[arg(long)]
        client_id: Option<String>,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// List lanes and their heads
    Lanes {
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Manage repo membership (readers/publishers)
    Members {
        #[command(subcommand)]
        command: MembersCommands,
    },

    /// Manage lane membership
    Lane {
        #[command(subcommand)]
        command: LaneCommands,
    },

    /// Fetch objects and publications from the configured remote
    Fetch {
        /// Fetch only this snap id
        #[arg(long)]
        snap_id: Option<String>,

        /// Fetch a specific bundle by id
        #[arg(long, conflicts_with_all = ["snap_id", "lane", "user", "release"])]
        bundle_id: Option<String>,

        /// Fetch the latest release from a channel
        #[arg(long, conflicts_with_all = ["snap_id", "lane", "user", "bundle_id"])]
        release: Option<String>,

        /// Fetch unpublished lane heads (defaults to publications if omitted)
        #[arg(long)]
        lane: Option<String>,

        /// Limit lane fetch to a specific user (defaults to all heads in lane)
        #[arg(long)]
        user: Option<String>,

        /// Materialize the fetched snap into a directory
        #[arg(long)]
        restore: bool,

        /// Directory to materialize into (defaults to a temp dir)
        #[arg(long)]
        into: Option<String>,

        /// Allow overwriting the destination directory
        #[arg(long)]
        force: bool,

        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Create a bundle on the remote from publications
    Bundle {
        /// Scope (defaults to remote config)
        #[arg(long)]
        scope: Option<String>,
        /// Gate (defaults to remote config)
        #[arg(long)]
        gate: Option<String>,
        /// Publication ids to include (repeatable). If omitted, includes all publications for scope+gate.
        #[arg(long = "publication")]
        publications: Vec<String>,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Promote a bundle to a downstream gate
    Promote {
        /// Bundle id to promote
        #[arg(long)]
        bundle_id: String,
        /// Downstream gate id
        #[arg(long)]
        to_gate: String,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Manage releases (named channels pointing at bundles)
    Release {
        #[command(subcommand)]
        command: ReleaseCommands,
    },

    /// Approve a bundle (manual policy step)
    Approve {
        /// Bundle id to approve
        #[arg(long)]
        bundle_id: String,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// List pinned bundles on the remote
    Pins {
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Pin or unpin a bundle on the remote
    Pin {
        /// Bundle id to pin/unpin
        #[arg(long)]
        bundle_id: String,
        /// Unpin instead of pin
        #[arg(long)]
        unpin: bool,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Show status for this workspace and remote
    Status {
        /// Emit JSON
        #[arg(long)]
        json: bool,
        /// Limit number of publications shown
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },

    /// Resolve superpositions by applying a saved resolution
    Resolve {
        #[command(subcommand)]
        command: ResolveCommands,
    },
}

#[derive(Subcommand)]
enum ReleaseCommands {
    /// Create a release in a channel from a bundle
    Create {
        #[arg(long)]
        channel: String,
        #[arg(long)]
        bundle_id: String,
        #[arg(long)]
        notes: Option<String>,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// List releases
    List {
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Show latest release in a channel
    Show {
        #[arg(long)]
        channel: String,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum MembersCommands {
    /// List repo members and roles
    List {
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Add a repo member
    Add {
        handle: String,
        /// Role: read|publish
        #[arg(long, default_value = "read")]
        role: String,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Remove a repo member
    Remove {
        handle: String,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum LaneCommands {
    /// Manage lane members
    Members {
        lane_id: String,
        #[command(subcommand)]
        command: LaneMembersCommands,
    },
}

#[derive(Subcommand)]
enum LaneMembersCommands {
    /// List lane members
    List {
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Add a lane member
    Add {
        handle: String,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Remove a lane member
    Remove {
        handle: String,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum ResolveCommands {
    /// Initialize a resolution file for a bundle (does not choose variants)
    Init {
        /// Bundle id to resolve
        #[arg(long)]
        bundle_id: String,
        /// Overwrite existing resolution
        #[arg(long)]
        force: bool,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Pick a variant for a conflicted path
    Pick {
        /// Bundle id
        #[arg(long)]
        bundle_id: String,
        /// Path to resolve (as shown in TUI)
        #[arg(long)]
        path: String,
        /// Variant number (1-based)
        #[arg(long, conflicts_with = "key")]
        variant: Option<u32>,

        /// Variant key JSON (stable)
        #[arg(long, conflicts_with = "variant")]
        key: Option<String>,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Clear a previously-picked variant for a conflicted path
    Clear {
        /// Bundle id
        #[arg(long)]
        bundle_id: String,
        /// Path to clear
        #[arg(long)]
        path: String,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Show the current resolution state
    Show {
        /// Bundle id
        #[arg(long)]
        bundle_id: String,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Validate a resolution against the current bundle root manifest
    Validate {
        /// Bundle id
        #[arg(long)]
        bundle_id: String,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Apply a resolution to a bundle root manifest and produce a new snap
    Apply {
        /// Bundle id to resolve
        #[arg(long)]
        bundle_id: String,
        /// Optional snap message
        #[arg(short = 'm', long)]
        message: Option<String>,
        /// Publish the resolved snap to current scope/gate
        #[arg(long)]
        publish: bool,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum TokenCommands {
    /// Create a new access token (shown once)
    Create {
        #[arg(long)]
        label: Option<String>,

        /// Create token for another user handle (admin)
        #[arg(long)]
        user: Option<String>,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// List your access tokens
    List {
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Revoke an access token
    Revoke {
        #[arg(long)]
        id: String,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum UserCommands {
    /// List users (admin)
    List {
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Create a user (admin)
    Create {
        handle: String,
        #[arg(long)]
        display_name: Option<String>,
        #[arg(long)]
        admin: bool,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum RemoteCommands {
    /// Show the configured remote
    Show {
        #[arg(long)]
        json: bool,
    },
    /// Set the configured remote
    Set {
        #[arg(long)]
        url: String,
        #[arg(long)]
        token: String,
        #[arg(long)]
        repo: String,
        #[arg(long, default_value = "main")]
        scope: String,
        #[arg(long, default_value = "dev-intake")]
        gate: String,
    },
    /// Create a repo on the remote (dev server convenience)
    CreateRepo {
        /// Repo id to create (defaults to configured remote repo)
        #[arg(long)]
        repo: Option<String>,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Purge remote objects/metadata (dev server)
    Purge {
        /// Dry run (default true)
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        dry_run: bool,

        /// Prune server metadata (default true)
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        prune_metadata: bool,

        /// Keep only the latest N releases per channel
        #[arg(long)]
        prune_releases_keep_last: Option<usize>,

        /// Emit JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum GateGraphCommands {
    /// Show the repo gate graph
    Show {
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Set the repo gate graph from a JSON file
    Set {
        #[arg(long)]
        file: PathBuf,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },

    /// Print a starter gate graph (and optionally apply it)
    Init {
        /// Apply to the remote repo (admin-only)
        #[arg(long)]
        apply: bool,
        /// Emit JSON
        #[arg(long)]
        json: bool,
    },
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{:#}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => {
            converge::tui::run()?;
        }
        Some(Commands::Init { force, path }) => {
            let root = path.unwrap_or(std::env::current_dir().context("get current dir")?);
            Workspace::init(&root, force)?;
            println!("Initialized Convergence workspace at {}", root.display());
        }
        Some(Commands::Snap { message, json }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let snap = ws.create_snap(message)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&snap).context("serialize snap json")?
                );
            } else {
                println!("{}", snap.id);
            }
        }
        Some(Commands::Snaps { json }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let snaps = ws.list_snaps()?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&snaps).context("serialize snaps json")?
                );
            } else {
                for snap in snaps {
                    let short = snap.id.chars().take(8).collect::<String>();
                    let msg = snap.message.unwrap_or_default();
                    if msg.is_empty() {
                        println!("{} {}", short, snap.created_at);
                    } else {
                        println!("{} {} {}", short, snap.created_at, msg);
                    }
                }
            }
        }
        Some(Commands::Show { snap_id, json }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let snap = ws.show_snap(&snap_id)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&snap).context("serialize snap")?
                );
            } else {
                println!("id: {}", snap.id);
                println!("created_at: {}", snap.created_at);
                if let Some(msg) = snap.message
                    && !msg.is_empty()
                {
                    println!("message: {}", msg);
                }
                println!("root_manifest: {}", snap.root_manifest.as_str());
                println!(
                    "stats: files={} dirs={} symlinks={} bytes={}",
                    snap.stats.files, snap.stats.dirs, snap.stats.symlinks, snap.stats.bytes
                );
            }
        }
        Some(Commands::Restore { snap_id, force }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            ws.restore_snap(&snap_id, force)?;
            println!("Restored {}", snap_id);
        }

        Some(Commands::Diff { from, to, json }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;

            let diffs = match (from.as_deref(), to.as_deref()) {
                (None, None) => {
                    let head = ws.store.get_head()?.context("no HEAD snap")?;
                    let head_snap = ws.store.get_snap(&head)?;
                    let from_tree =
                        converge::diff::tree_from_store(&ws.store, &head_snap.root_manifest)?;

                    let (cur_root, cur_manifests, _stats) = ws.current_manifest_tree()?;
                    let to_tree = converge::diff::tree_from_memory(&cur_manifests, &cur_root)?;

                    converge::diff::diff_trees(&from_tree, &to_tree)
                }
                (Some(_), None) | (None, Some(_)) => {
                    anyhow::bail!(
                        "use both --from and --to for snap diffs, or omit both for workspace vs HEAD"
                    )
                }
                (Some(from), Some(to)) => {
                    let from_snap = ws.store.get_snap(from)?;
                    let to_snap = ws.store.get_snap(to)?;
                    let from_tree =
                        converge::diff::tree_from_store(&ws.store, &from_snap.root_manifest)?;
                    let to_tree =
                        converge::diff::tree_from_store(&ws.store, &to_snap.root_manifest)?;
                    converge::diff::diff_trees(&from_tree, &to_tree)
                }
            };

            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&diffs).context("serialize diff json")?
                );
            } else {
                for d in &diffs {
                    match d {
                        converge::diff::DiffLine::Added { path, .. } => println!("A {}", path),
                        converge::diff::DiffLine::Deleted { path, .. } => println!("D {}", path),
                        converge::diff::DiffLine::Modified { path, .. } => println!("M {}", path),
                    }
                }
                println!("{} changes", diffs.len());
            }
        }

        Some(Commands::Mv { from, to }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            ws.move_path(std::path::Path::new(&from), std::path::Path::new(&to))?;
            println!("Moved {} -> {}", from, to);
        }
        Some(Commands::Remote { command }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            cli_exec::handle_remote_command(&ws, command)?;
        }

        Some(Commands::Gates { command }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            cli_exec::handle_gates_command(&ws, command)?;
        }

        Some(Commands::Login {
            url,
            token,
            repo,
            scope,
            gate,
        }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let mut cfg = ws.store.read_config()?;
            let remote = RemoteConfig {
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
            println!("Logged in");
        }

        Some(Commands::Logout) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let remote = require_remote(&ws.store)?;
            ws.store
                .clear_remote_token(&remote)
                .context("clear remote token")?;
            println!("Logged out");
        }

        Some(Commands::Whoami { json }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let (remote, token) = require_remote_and_token(&ws.store)?;
            let client = RemoteClient::new(remote, token)?;
            let who = client.whoami()?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&who).context("serialize whoami json")?
                );
            } else {
                println!("user: {}", who.user);
                println!("user_id: {}", who.user_id);
                println!("admin: {}", who.admin);
            }
        }

        Some(Commands::Token { command }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            cli_exec::handle_token_command(&ws, command)?;
        }

        Some(Commands::User { command }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            cli_exec::handle_user_command(&ws, command)?;
        }
        Some(Commands::Publish {
            snap_id,
            scope,
            gate,
            metadata_only,
            json,
        }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let (remote, token) = require_remote_and_token(&ws.store)?;
            let client = RemoteClient::new(remote.clone(), token)?;

            let snap = match snap_id {
                Some(id) => ws.show_snap(&id)?,
                None => ws
                    .list_snaps()?
                    .into_iter()
                    .next()
                    .context("no snaps found (run `converge snap`)")?,
            };

            let scope = scope.unwrap_or_else(|| remote.scope.clone());
            let gate = gate.unwrap_or_else(|| remote.gate.clone());

            let pubrec = if metadata_only {
                client.publish_snap_metadata_only(&ws.store, &snap, &scope, &gate)?
            } else {
                client.publish_snap(&ws.store, &snap, &scope, &gate)?
            };

            ws.store
                .set_last_published(&remote, &scope, &gate, &snap.id)
                .context("record last published snap")?;

            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&pubrec).context("serialize publish json")?
                );
            } else {
                println!("Published {}", snap.id);
            }
        }

        Some(Commands::Sync {
            snap_id,
            lane,
            client_id,
            json,
        }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let (remote, token) = require_remote_and_token(&ws.store)?;
            let client = RemoteClient::new(remote.clone(), token)?;

            let snap = match snap_id {
                Some(id) => ws.show_snap(&id)?,
                None => ws
                    .list_snaps()?
                    .into_iter()
                    .next()
                    .context("no snaps to sync")?,
            };

            let head = client.sync_snap(&ws.store, &snap, &lane, client_id)?;

            ws.store
                .set_lane_sync(&lane, &snap.id, &head.updated_at)
                .context("record lane sync")?;

            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&head).context("serialize sync json")?
                );
            } else {
                println!("Synced {} to lane {}", snap.id, lane);
            }
        }

        Some(Commands::Lanes { json }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let (remote, token) = require_remote_and_token(&ws.store)?;
            let client = RemoteClient::new(remote, token)?;
            let mut lanes = client.list_lanes()?;
            lanes.sort_by(|a, b| a.id.cmp(&b.id));

            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&lanes).context("serialize lanes json")?
                );
            } else {
                for l in lanes {
                    println!("lane: {}", l.id);
                    let mut members = l.members.into_iter().collect::<Vec<_>>();
                    members.sort();
                    for m in members {
                        if let Some(h) = l.heads.get(&m) {
                            let short = h.snap_id.chars().take(8).collect::<String>();
                            println!("  {} {} {}", m, short, h.updated_at);
                        } else {
                            println!("  {} (no head)", m);
                        }
                    }
                }
            }
        }

        Some(Commands::Members { command }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            cli_exec::handle_members_command(&ws, command)?;
        }

        Some(Commands::Lane { command }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            cli_exec::handle_lane_command(&ws, command)?;
        }

        Some(Commands::Fetch {
            snap_id,
            bundle_id,
            release,
            lane,
            user,
            restore,
            into,
            force,
            json,
        }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let (remote, token) = require_remote_and_token(&ws.store)?;
            let client = RemoteClient::new(remote, token)?;

            if let Some(bundle_id) = bundle_id.as_deref() {
                let bundle = client.get_bundle(bundle_id)?;
                let root = converge::model::ObjectId(bundle.root_manifest.clone());
                client.fetch_manifest_tree(&ws.store, &root)?;

                let mut restored_to: Option<String> = None;
                if restore {
                    let dest = if let Some(p) = into.as_deref() {
                        std::path::PathBuf::from(p)
                    } else {
                        let short = bundle_id.chars().take(8).collect::<String>();
                        let nanos = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_nanos();
                        std::env::temp_dir()
                            .join(format!("converge-grab-bundle-{}-{}", short, nanos))
                    };

                    ws.materialize_manifest_to(&root, &dest, force)
                        .with_context(|| format!("materialize bundle to {}", dest.display()))?;
                    restored_to = Some(dest.display().to_string());
                    if !json {
                        println!("Materialized bundle {} into {}", bundle_id, dest.display());
                    }
                }

                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "kind": "bundle",
                            "bundle_id": bundle.id,
                            "root_manifest": bundle.root_manifest,
                            "restored_to": restored_to,
                        }))
                        .context("serialize fetch bundle json")?
                    );
                } else {
                    println!("Fetched bundle {}", bundle.id);
                }
                return Ok(());
            }

            if let Some(channel) = release.as_deref() {
                let rel = client.get_release(channel)?;
                let bundle = client.get_bundle(&rel.bundle_id)?;
                let root = converge::model::ObjectId(bundle.root_manifest.clone());
                client.fetch_manifest_tree(&ws.store, &root)?;

                let mut restored_to: Option<String> = None;
                if restore {
                    let dest = if let Some(p) = into.as_deref() {
                        std::path::PathBuf::from(p)
                    } else {
                        let short = rel.bundle_id.chars().take(8).collect::<String>();
                        let nanos = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_nanos();
                        std::env::temp_dir()
                            .join(format!("converge-grab-release-{}-{}", short, nanos))
                    };

                    ws.materialize_manifest_to(&root, &dest, force)
                        .with_context(|| format!("materialize release to {}", dest.display()))?;
                    restored_to = Some(dest.display().to_string());
                    if !json {
                        println!(
                            "Materialized release {} (bundle {}) into {}",
                            rel.channel,
                            rel.bundle_id,
                            dest.display()
                        );
                    }
                }

                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "kind": "release",
                            "channel": rel.channel,
                            "bundle_id": rel.bundle_id,
                            "root_manifest": bundle.root_manifest,
                            "restored_to": restored_to,
                        }))
                        .context("serialize fetch release json")?
                    );
                } else {
                    println!("Fetched release {} ({})", rel.channel, rel.bundle_id);
                }
                return Ok(());
            }

            let fetched = if let Some(lane) = lane.as_deref() {
                client.fetch_lane_heads(&ws.store, lane, user.as_deref())?
            } else {
                client.fetch_publications(&ws.store, snap_id.as_deref())?
            };

            if restore {
                let snap_to_restore = if let Some(id) = snap_id.as_deref() {
                    id.to_string()
                } else if fetched.len() == 1 {
                    fetched[0].clone()
                } else {
                    anyhow::bail!(
                        "--restore requires a specific snap (use --snap-id, or use --user so only one lane head is fetched)"
                    );
                };

                let dest = if let Some(p) = into.as_deref() {
                    std::path::PathBuf::from(p)
                } else {
                    let short = snap_to_restore.chars().take(8).collect::<String>();
                    let nanos = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_nanos();
                    std::env::temp_dir().join(format!("converge-grab-{}-{}", short, nanos))
                };

                ws.materialize_snap_to(&snap_to_restore, &dest, force)
                    .with_context(|| format!("materialize snap to {}", dest.display()))?;
                if !json {
                    println!("Materialized {} into {}", snap_to_restore, dest.display());
                }
            }

            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&fetched).context("serialize fetch json")?
                );
            } else {
                for id in fetched {
                    println!("Fetched {}", id);
                }
            }
        }
        Some(Commands::Bundle {
            scope,
            gate,
            publications,
            json,
        }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let (remote, token) = require_remote_and_token(&ws.store)?;
            let client = RemoteClient::new(remote.clone(), token)?;
            let scope = scope.unwrap_or_else(|| remote.scope.clone());
            let gate = gate.unwrap_or_else(|| remote.gate.clone());

            let pubs = if publications.is_empty() {
                let all = client.list_publications()?;
                all.into_iter()
                    .filter(|p| p.scope == scope && p.gate == gate)
                    .map(|p| p.id)
                    .collect::<Vec<_>>()
            } else {
                publications
            };

            if pubs.is_empty() {
                anyhow::bail!(
                    "no publications found for scope={} gate={} (publish first)",
                    scope,
                    gate
                );
            }

            let bundle = client.create_bundle(&scope, &gate, &pubs)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&bundle).context("serialize bundle json")?
                );
            } else {
                println!("{}", bundle.id);
            }
        }
        Some(Commands::Promote {
            bundle_id,
            to_gate,
            json,
        }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let (remote, token) = require_remote_and_token(&ws.store)?;
            let client = RemoteClient::new(remote, token)?;
            let promotion = client.promote_bundle(&bundle_id, &to_gate)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&promotion).context("serialize promotion json")?
                );
            } else {
                println!("Promoted {} -> {}", promotion.from_gate, promotion.to_gate);
            }
        }

        Some(Commands::Release { command }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            cli_exec::handle_release_command(&ws, command)?;
        }
        Some(Commands::Approve { bundle_id, json }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let (remote, token) = require_remote_and_token(&ws.store)?;
            let client = RemoteClient::new(remote, token)?;
            let bundle = client.approve_bundle(&bundle_id)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&bundle).context("serialize approve json")?
                );
            } else if bundle.promotable {
                println!("Approved {} (now promotable)", bundle.id);
            } else {
                println!(
                    "Approved {} (still blocked: {:?})",
                    bundle.id, bundle.reasons
                );
            }
        }
        Some(Commands::Pins { json }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let (remote, token) = require_remote_and_token(&ws.store)?;
            let client = RemoteClient::new(remote, token)?;
            let pins = client.list_pins()?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&pins).context("serialize pins json")?
                );
            } else {
                for b in pins.bundles {
                    println!("{}", b);
                }
            }
        }
        Some(Commands::Pin {
            bundle_id,
            unpin,
            json,
        }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let (remote, token) = require_remote_and_token(&ws.store)?;
            let client = RemoteClient::new(remote, token)?;
            if unpin {
                client.unpin_bundle(&bundle_id)?;
            } else {
                client.pin_bundle(&bundle_id)?;
            }
            if json {
                println!(
                    "{}",
                    serde_json::json!({
                        "bundle_id": bundle_id,
                        "pinned": !unpin
                    })
                );
            } else if unpin {
                println!("Unpinned {}", bundle_id);
            } else {
                println!("Pinned {}", bundle_id);
            }
        }
        Some(Commands::Status { json, limit }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            let cfg = ws.store.read_config()?;
            let Some(remote) = cfg.remote else {
                if json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({"remote": null}))
                            .context("serialize status json")?
                    );
                } else {
                    println!("No remote configured");
                }
                return Ok(());
            };

            let token = ws
                .store
                .get_remote_token(&remote)?
                .context(
                    "no remote token configured (run `converge login --url ... --token ... --repo ...`)",
                )?;
            let client = RemoteClient::new(remote.clone(), token)?;
            let mut pubs = client.list_publications()?;
            pubs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            pubs.truncate(limit);
            let promotion_state = client.promotion_state(&remote.scope)?;
            let releases = client.list_releases().unwrap_or_default();

            let mut latest_by_channel: std::collections::BTreeMap<
                String,
                converge::remote::Release,
            > = std::collections::BTreeMap::new();
            for r in releases {
                match latest_by_channel.get(&r.channel) {
                    None => {
                        latest_by_channel.insert(r.channel.clone(), r);
                    }
                    Some(prev) => {
                        if r.released_at > prev.released_at {
                            latest_by_channel.insert(r.channel.clone(), r);
                        }
                    }
                }
            }

            if json {
                let remote_json = serde_json::json!({
                    "base_url": remote.base_url.as_str(),
                    "repo_id": remote.repo_id.as_str(),
                    "scope": remote.scope.as_str(),
                    "gate": remote.gate.as_str(),
                });
                let pubs_json = pubs
                    .into_iter()
                    .map(|p| {
                        let present = ws.store.has_snap(&p.snap_id);
                        serde_json::json!({
                            "id": p.id,
                            "snap_id": p.snap_id,
                            "scope": p.scope,
                            "gate": p.gate,
                            "publisher": p.publisher,
                            "created_at": p.created_at,
                            "local_present": present
                        })
                    })
                    .collect::<Vec<_>>();
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "remote": remote_json,
                        "publications": pubs_json,
                        "promotion_state": promotion_state,
                        "releases": latest_by_channel.values().collect::<Vec<_>>()
                    }))
                    .context("serialize status json")?
                );
            } else {
                println!("remote: {}", remote.base_url);
                println!("repo: {}", remote.repo_id);
                println!("scope: {}", remote.scope);
                println!("gate: {}", remote.gate);

                println!("releases:");
                if latest_by_channel.is_empty() {
                    println!("(none)");
                } else {
                    for (ch, r) in &latest_by_channel {
                        let short = r.bundle_id.chars().take(8).collect::<String>();
                        println!("{} {} {} {}", ch, short, r.released_at, r.released_by);
                    }
                }

                println!("promotion_state:");
                if promotion_state.is_empty() {
                    println!("(none)");
                } else {
                    let mut keys = promotion_state.keys().cloned().collect::<Vec<_>>();
                    keys.sort();
                    for gate in keys {
                        let bid = promotion_state.get(&gate).cloned().unwrap_or_default();
                        let short = bid.chars().take(8).collect::<String>();
                        println!("{} {}", gate, short);
                    }
                }
                println!("publications:");
                for p in pubs {
                    let short = p.snap_id.chars().take(8).collect::<String>();
                    let present = if ws.store.has_snap(&p.snap_id) {
                        "local"
                    } else {
                        "missing"
                    };
                    println!(
                        "{} {} {} {} {}",
                        short, p.created_at, p.publisher, p.scope, present
                    );
                }
            }
        }

        Some(Commands::Resolve { command }) => {
            let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
            cli_exec::handle_resolve_command(&ws, command)?;
        }
    }

    Ok(())
}

fn require_remote(store: &LocalStore) -> Result<RemoteConfig> {
    let cfg = store.read_config()?;
    cfg.remote
        .context("no remote configured (run `converge login --url ... --token ... --repo ...`)")
}

fn require_remote_and_token(store: &LocalStore) -> Result<(RemoteConfig, String)> {
    let remote = require_remote(store)?;
    let token = store.get_remote_token(&remote)?.context(
        "no remote token configured (run `converge login --url ... --token ... --repo ...`)",
    )?;
    Ok((remote, token))
}
