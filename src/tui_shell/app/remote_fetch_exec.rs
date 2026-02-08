use super::remote_fetch_parse::{FetchSpec, parse_fetch_spec};
use super::*;

impl App {
    pub(in crate::tui_shell) fn cmd_fetch_impl(&mut self, args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };
        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };

        let parsed = match parse_fetch_spec(args) {
            Ok(p) => p,
            Err(msg) => {
                self.push_error(msg);
                return;
            }
        };

        if let Some(bundle_id) = parsed.bundle_id.as_deref() {
            self.fetch_bundle(&client, &ws, &parsed, bundle_id);
            return;
        }

        if let Some(channel) = parsed.release.as_deref() {
            self.fetch_release(&client, &ws, &parsed, channel);
            return;
        }

        self.fetch_snaps(&client, &ws, &parsed);
    }

    fn fetch_bundle(
        &mut self,
        client: &RemoteClient,
        ws: &Workspace,
        parsed: &FetchSpec,
        bundle_id: &str,
    ) {
        let bundle = match client.get_bundle(bundle_id) {
            Ok(b) => b,
            Err(err) => {
                self.push_error(format!("get bundle: {:#}", err));
                return;
            }
        };
        let root = crate::model::ObjectId(bundle.root_manifest.clone());
        if let Err(err) = client.fetch_manifest_tree(&ws.store, &root) {
            self.push_error(format!("fetch bundle objects: {:#}", err));
            return;
        }

        if parsed.restore {
            let dest = parsed
                .into
                .as_deref()
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| {
                    let short = bundle.id.chars().take(8).collect::<String>();
                    temp_restore_path("bundle", &short)
                });
            if let Err(err) = ws.materialize_manifest_to(&root, &dest, parsed.force) {
                self.push_error(format!("restore: {:#}", err));
                return;
            }
            self.push_output(vec![format!(
                "materialized bundle {} into {}",
                bundle.id,
                dest.display()
            )]);
        } else {
            self.push_output(vec![format!("fetched bundle {}", bundle.id)]);
        }
        self.refresh_root_view();
    }

    fn fetch_release(
        &mut self,
        client: &RemoteClient,
        ws: &Workspace,
        parsed: &FetchSpec,
        channel: &str,
    ) {
        let rel = match client.get_release(channel) {
            Ok(r) => r,
            Err(err) => {
                self.push_error(format!("get release: {:#}", err));
                return;
            }
        };
        let bundle = match client.get_bundle(&rel.bundle_id) {
            Ok(b) => b,
            Err(err) => {
                self.push_error(format!("get bundle: {:#}", err));
                return;
            }
        };

        let root = crate::model::ObjectId(bundle.root_manifest.clone());
        if let Err(err) = client.fetch_manifest_tree(&ws.store, &root) {
            self.push_error(format!("fetch release objects: {:#}", err));
            return;
        }

        if parsed.restore {
            let dest = parsed
                .into
                .as_deref()
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| {
                    let short = rel.bundle_id.chars().take(8).collect::<String>();
                    temp_restore_path("release", &short)
                });
            if let Err(err) = ws.materialize_manifest_to(&root, &dest, parsed.force) {
                self.push_error(format!("restore: {:#}", err));
                return;
            }
            self.push_output(vec![format!(
                "materialized release {} ({}) into {}",
                rel.channel,
                rel.bundle_id,
                dest.display()
            )]);
        } else {
            self.push_output(vec![format!(
                "fetched release {} ({})",
                rel.channel, rel.bundle_id
            )]);
        }
        self.refresh_root_view();
    }

    fn fetch_snaps(&mut self, client: &RemoteClient, ws: &Workspace, parsed: &FetchSpec) {
        let res = if let Some(lane) = parsed.lane.as_deref() {
            client.fetch_lane_heads(&ws.store, lane, parsed.user.as_deref())
        } else {
            client.fetch_publications(&ws.store, parsed.snap_id.as_deref())
        };

        match res {
            Ok(fetched) => {
                self.push_output(vec![format!("fetched {} snaps", fetched.len())]);
                self.refresh_root_view();

                // If we're looking at lanes, update local markers.
                if self.mode() == UiMode::Lanes
                    && let Some(v) = self.current_view_mut::<LanesView>()
                {
                    for it in &mut v.items {
                        if let Some(h) = &it.head {
                            it.local = ws.store.has_snap(&h.snap_id);
                        }
                    }
                    v.updated_at = now_ts();
                }
            }
            Err(err) => {
                self.push_error(format!("fetch: {:#}", err));
            }
        }
    }
}

fn temp_restore_path(kind: &str, id_prefix: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("converge-grab-{}-{}-{}", kind, id_prefix, nanos))
}
