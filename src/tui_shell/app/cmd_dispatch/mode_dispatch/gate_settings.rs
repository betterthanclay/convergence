use super::*;

pub(super) fn dispatch_gate_graph_mode(app: &mut App, mode: UiMode, cmd: &str, args: &[String]) {
    match cmd {
        "back" => app.dispatch_mode_back(),
        "refresh" | "r" => {
            let _ = args;
            app.open_gate_graph_view();
        }
        "add-gate" => {
            let _ = args;
            app.cmd_gate_graph_add_gate();
        }
        "remove-gate" => {
            let _ = args;
            app.cmd_gate_graph_remove_gate();
        }
        "edit-upstream" => {
            let _ = args;
            app.cmd_gate_graph_edit_upstream();
        }
        "set-approvals" => {
            let _ = args;
            app.cmd_gate_graph_set_approvals();
        }
        "toggle-releases" => {
            let _ = args;
            app.cmd_gate_graph_toggle_releases();
        }
        "toggle-superpositions" => {
            let _ = args;
            app.cmd_gate_graph_toggle_superpositions();
        }
        "toggle-metadata-only" => {
            let _ = args;
            app.cmd_gate_graph_toggle_metadata_only();
        }
        _ => app.push_unknown_mode_command(mode, cmd, args),
    }
}

pub(super) fn dispatch_settings_mode(app: &mut App, mode: UiMode, cmd: &str, args: &[String]) {
    match cmd {
        "back" => app.dispatch_mode_back(),
        "do" => {
            if !args.is_empty() {
                app.push_error("usage: do".to_string());
                return;
            }
            app.cmd_settings_do_mode();
        }
        _ => app.push_unknown_mode_command(mode, cmd, args),
    }
}
