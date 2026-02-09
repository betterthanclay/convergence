use super::*;

pub(super) fn dispatch_inbox_mode(app: &mut App, mode: UiMode, cmd: &str, args: &[String]) {
    match cmd {
        "back" => app.dispatch_mode_back(),
        "edit" => {
            if !args.is_empty() {
                app.push_error("usage: edit".to_string());
                return;
            }
            app.start_browse_wizard(BrowseTarget::Inbox);
        }
        "bundle" => app.cmd_inbox_bundle_mode(args),
        "fetch" => app.cmd_inbox_fetch_mode(args),
        _ => app.push_unknown_mode_command(mode, cmd, args),
    }
}

pub(super) fn dispatch_bundles_mode(app: &mut App, mode: UiMode, cmd: &str, args: &[String]) {
    match cmd {
        "back" => app.dispatch_mode_back(),
        "edit" => {
            if !args.is_empty() {
                app.push_error("usage: edit".to_string());
                return;
            }
            app.start_browse_wizard(BrowseTarget::Bundles);
        }
        "approve" => app.cmd_bundles_approve_mode(args),
        "pin" => app.cmd_bundles_pin_mode(args),
        "promote" => app.cmd_bundles_promote_mode(args),
        "release" => app.cmd_bundles_release_mode(args),
        "superpositions" | "supers" => app.cmd_bundles_superpositions_mode(args),
        _ => app.push_unknown_mode_command(mode, cmd, args),
    }
}

pub(super) fn dispatch_releases_mode(app: &mut App, mode: UiMode, cmd: &str, args: &[String]) {
    match cmd {
        "back" => app.dispatch_mode_back(),
        "fetch" => app.cmd_releases_fetch_mode(args),
        _ => app.push_unknown_mode_command(mode, cmd, args),
    }
}

pub(super) fn dispatch_lanes_mode(app: &mut App, mode: UiMode, cmd: &str, args: &[String]) {
    match cmd {
        "back" => app.dispatch_mode_back(),
        "fetch" => app.cmd_lanes_fetch_mode(args),
        _ => app.push_unknown_mode_command(mode, cmd, args),
    }
}

pub(super) fn dispatch_superpositions_mode(
    app: &mut App,
    mode: UiMode,
    cmd: &str,
    args: &[String],
) {
    match cmd {
        "back" => app.dispatch_mode_back(),
        "pick" => app.cmd_superpositions_pick_mode(args),
        "clear" => app.cmd_superpositions_clear_mode(args),
        "next-missing" => app.cmd_superpositions_next_missing_mode(args),
        "next-invalid" => app.cmd_superpositions_next_invalid_mode(args),
        "validate" => app.cmd_superpositions_validate_mode(args),
        "apply" => app.cmd_superpositions_apply_mode(args),
        _ => app.push_unknown_mode_command(mode, cmd, args),
    }
}
