use super::*;

pub(super) fn is_prompt_first_subcommand(sub: &str) -> bool {
    matches!(sub, "add" | "remove" | "rm")
}

pub(super) fn handle_prompt_first(app: &mut App, args: &[String]) {
    // Prompt-first UX:
    // - `lane-member` -> wizard
    // - `lane-member add` / `lane-member remove` -> wizard
    // - `lane-member add <lane> <handle>`
    // - `lane-member remove <lane> <handle>`
    let sub = args[0].as_str();
    let action = if sub == "add" {
        Some(MemberAction::Add)
    } else {
        Some(MemberAction::Remove)
    };
    if args.len() < 3 {
        app.start_lane_member_wizard(action);
        return;
    }
    let lane = args[1].trim().to_string();
    let handle = args[2].trim().to_string();
    if lane.is_empty() || handle.is_empty() {
        app.start_lane_member_wizard(action);
        return;
    }

    let client = match app.remote_client() {
        Some(c) => c,
        None => {
            app.start_login_wizard();
            return;
        }
    };
    match action {
        Some(MemberAction::Add) => match client.add_lane_member(&lane, &handle) {
            Ok(()) => {
                app.push_output(vec![format!("added {} to lane {}", handle, lane)]);
                app.refresh_root_view();
            }
            Err(err) => app.push_error(format!("lane-member add: {:#}", err)),
        },
        Some(MemberAction::Remove) => match client.remove_lane_member(&lane, &handle) {
            Ok(()) => {
                app.push_output(vec![format!("removed {} from lane {}", handle, lane)]);
                app.refresh_root_view();
            }
            Err(err) => app.push_error(format!("lane-member remove: {:#}", err)),
        },
        None => app.start_lane_member_wizard(None),
    }
}
