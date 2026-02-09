use super::*;

impl App {
    pub(in crate::tui_shell) fn cmd_superpositions_validate_mode(&mut self, args: &[String]) {
        if !args.is_empty() {
            self.push_error("usage: validate".to_string());
            return;
        }

        let Some(ws) = self.require_workspace() else {
            return;
        };

        let out: std::result::Result<String, String> = match self
            .current_view_mut::<SuperpositionsView>()
        {
            Some(v) => {
                v.validation = validate_resolution(&ws.store, &v.root_manifest, &v.decisions).ok();
                v.updated_at = now_ts();
                let ok = v.validation.as_ref().is_some_and(|r| r.ok);
                Ok(format!("validation: {}", if ok { "ok" } else { "invalid" }))
            }
            None => Err("not in superpositions mode".to_string()),
        };

        match out {
            Ok(line) => self.push_output(vec![line]),
            Err(err) => self.push_error(err),
        }
    }

    pub(in crate::tui_shell) fn cmd_superpositions_apply_mode(&mut self, args: &[String]) {
        let mut publish = false;
        for a in args {
            match a.as_str() {
                "--publish" | "publish" => publish = true,
                _ => {
                    self.push_error("usage: apply [publish]".to_string());
                    return;
                }
            }
        }

        let Some(ws) = self.require_workspace() else {
            return;
        };

        let Some((bundle_id, root_manifest)) = self
            .current_view::<SuperpositionsView>()
            .map(|v| (v.bundle_id.clone(), v.root_manifest.clone()))
        else {
            self.push_error("not in superpositions mode".to_string());
            return;
        };

        let resolution = match ws.store.get_resolution(&bundle_id) {
            Ok(r) => r,
            Err(err) => {
                self.push_error(format!("load resolution: {:#}", err));
                return;
            }
        };
        if resolution.root_manifest != root_manifest {
            self.push_error("resolution root_manifest mismatch".to_string());
            return;
        }

        let resolved_root = match crate::resolve::apply_resolution(
            &ws.store,
            &root_manifest,
            &resolution.decisions,
        ) {
            Ok(r) => r,
            Err(err) => {
                self.push_error(format!("apply resolution: {:#}", err));
                return;
            }
        };

        let created_at = now_ts();
        let snap_id = crate::model::compute_snap_id(&created_at, &resolved_root);
        let snap = crate::model::SnapRecord {
            version: 1,
            id: snap_id,
            created_at: created_at.clone(),
            root_manifest: resolved_root,
            message: None,
            stats: crate::model::SnapStats::default(),
        };

        if let Err(err) = ws.store.put_snap(&snap) {
            self.push_error(format!("write snap: {:#}", err));
            return;
        }

        let mut pub_id: Option<String> = None;
        if publish {
            let remote = match self.remote_config() {
                Some(r) => r,
                None => {
                    self.push_error("no remote configured".to_string());
                    return;
                }
            };

            let token = match ws.store.get_remote_token(&remote) {
                Ok(Some(t)) => t,
                Ok(None) => {
                    self.push_error(
                        "no remote token configured (run `login --url ... --token ... --repo ...`)"
                            .to_string(),
                    );
                    return;
                }
                Err(err) => {
                    self.push_error(format!("read remote token: {:#}", err));
                    return;
                }
            };

            let client = match RemoteClient::new(remote.clone(), token) {
                Ok(c) => c,
                Err(err) => {
                    self.push_error(format!("init remote client: {:#}", err));
                    return;
                }
            };

            let res_meta = crate::remote::PublicationResolution {
                bundle_id: bundle_id.clone(),
                root_manifest: root_manifest.as_str().to_string(),
                resolved_root_manifest: snap.root_manifest.as_str().to_string(),
                created_at: snap.created_at.clone(),
            };

            match client.publish_snap_with_resolution(
                &ws.store,
                &snap,
                &remote.scope,
                &remote.gate,
                Some(res_meta),
            ) {
                Ok(p) => pub_id = Some(p.id),
                Err(err) => {
                    self.push_error(format!("publish: {:#}", err));
                    return;
                }
            }
        }

        if let Some(pid) = pub_id {
            self.push_output(vec![format!(
                "resolved snap {} (published {})",
                snap.id, pid
            )]);
        } else {
            self.push_output(vec![format!("resolved snap {}", snap.id)]);
        }
    }
}
