use super::*;

impl App {
    fn load_settings_snapshot(&mut self) -> Option<SettingsSnapshot> {
        let ws = self.workspace.as_ref()?;

        let cfg = match ws.store.read_config() {
            Ok(c) => c,
            Err(err) => {
                self.push_error(format!("read config: {:#}", err));
                return None;
            }
        };

        let (chunk_size, threshold) = cfg
            .chunking
            .as_ref()
            .map(|c| (c.chunk_size, c.threshold))
            .unwrap_or((4 * 1024 * 1024, 8 * 1024 * 1024));

        let r = cfg.retention.unwrap_or_default();
        Some(SettingsSnapshot {
            workflow_profile: cfg.workflow_profile,
            chunk_size_mib: chunk_size / (1024 * 1024),
            threshold_mib: threshold / (1024 * 1024),

            retention_keep_last: r.keep_last,
            retention_keep_days: r.keep_days,
            retention_prune_snaps: r.prune_snaps,
            retention_pinned: r.pinned.len(),
        })
    }

    pub(super) fn refresh_settings_view(&mut self) {
        let snapshot = self.load_settings_snapshot();
        let Some(v) = self.current_view_mut::<SettingsView>() else {
            return;
        };
        v.snapshot = snapshot;
        v.updated_at = now_ts();
    }

    pub(super) fn cmd_settings(&mut self, args: &[String]) {
        if !args.is_empty() {
            self.push_error("usage: settings".to_string());
            return;
        }

        if self.mode() == UiMode::Settings {
            self.refresh_settings_view();
            self.push_output(vec!["refreshed settings".to_string()]);
            return;
        }

        let snapshot = self.load_settings_snapshot();
        let mut items = vec![
            SettingsItemKind::ToggleTimestamps,
            SettingsItemKind::WorkflowProfileSet,
        ];
        if snapshot.is_some() {
            items.extend([
                SettingsItemKind::ChunkingShow,
                SettingsItemKind::ChunkingSet,
                SettingsItemKind::ChunkingReset,
                SettingsItemKind::RetentionShow,
                SettingsItemKind::RetentionKeepLast,
                SettingsItemKind::RetentionKeepDays,
                SettingsItemKind::ToggleRetentionPruneSnaps,
                SettingsItemKind::RetentionReset,
            ]);
        }

        self.push_view(SettingsView {
            updated_at: now_ts(),
            items,
            selected: 0,
            snapshot,
        });
        self.push_output(vec!["opened settings".to_string()]);
    }
}
