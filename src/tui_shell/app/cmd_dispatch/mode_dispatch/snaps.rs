use super::*;

pub(super) fn dispatch_snaps_mode(app: &mut App, mode: UiMode, cmd: &str, args: &[String]) {
    match cmd {
        "back" => app.dispatch_mode_back(),
        "filter" => app.cmd_snaps_filter(args),
        "clear-filter" => app.cmd_snaps_clear_filter(args),
        "snap" => app.cmd_snaps_snap(args),
        "msg" => app.cmd_snaps_msg(args),
        "revert" => app.cmd_snaps_revert(args),
        "unsnap" => app.cmd_snaps_unsnap(args),
        "restore" => app.cmd_snaps_restore(args),
        _ => app.push_unknown_mode_command(mode, cmd, args),
    }
}
