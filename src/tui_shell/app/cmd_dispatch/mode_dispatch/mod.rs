use super::super::*;

mod gate_settings;
mod remote_modes;
mod snaps;

impl App {
    pub(in crate::tui_shell::app) fn dispatch_mode(
        &mut self,
        mode: UiMode,
        cmd: &str,
        args: &[String],
    ) {
        match mode {
            UiMode::Snaps => snaps::dispatch_snaps_mode(self, mode, cmd, args),
            UiMode::Inbox => remote_modes::dispatch_inbox_mode(self, mode, cmd, args),
            UiMode::Bundles => remote_modes::dispatch_bundles_mode(self, mode, cmd, args),
            UiMode::Releases => remote_modes::dispatch_releases_mode(self, mode, cmd, args),
            UiMode::Lanes => remote_modes::dispatch_lanes_mode(self, mode, cmd, args),
            UiMode::Superpositions => {
                remote_modes::dispatch_superpositions_mode(self, mode, cmd, args)
            }
            UiMode::GateGraph => gate_settings::dispatch_gate_graph_mode(self, mode, cmd, args),
            UiMode::Settings => gate_settings::dispatch_settings_mode(self, mode, cmd, args),
            UiMode::Root => {
                self.dispatch_root(cmd, args);
            }
        }
    }

    pub(super) fn dispatch_mode_back(&mut self) {
        self.pop_mode();
        self.push_output(vec!["back".to_string()]);
    }

    pub(super) fn push_unknown_mode_command(&mut self, mode: UiMode, cmd: &str, args: &[String]) {
        if !self.dispatch_global(cmd, args) {
            self.push_error(format!(
                "unknown command in {:?} mode: {} (try /help)",
                mode, cmd
            ));
        }
    }
}
