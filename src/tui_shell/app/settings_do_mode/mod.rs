use super::*;

mod chunking;
mod profile;
mod retention;
mod toggles;

impl App {
    pub(super) fn cmd_settings_do_mode(&mut self) {
        let Some(kind) = self
            .current_view::<SettingsView>()
            .and_then(|v| v.selected_kind())
        else {
            self.push_error("no selected setting".to_string());
            return;
        };

        match kind {
            SettingsItemKind::ToggleTimestamps => toggles::toggle_timestamps(self),
            SettingsItemKind::WorkflowProfileSet => profile::set(self),
            SettingsItemKind::ChunkingShow => chunking::show(self),
            SettingsItemKind::ChunkingSet => chunking::set(self),
            SettingsItemKind::ChunkingReset => chunking::reset(self),
            SettingsItemKind::RetentionShow => retention::show(self),
            SettingsItemKind::RetentionKeepLast => retention::keep_last(self),
            SettingsItemKind::RetentionKeepDays => retention::keep_days(self),
            SettingsItemKind::ToggleRetentionPruneSnaps => retention::toggle_prune_snaps(self),
            SettingsItemKind::RetentionReset => retention::reset(self),
        }
    }
}
