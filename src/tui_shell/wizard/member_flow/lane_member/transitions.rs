use super::MemberAction;
use super::prompts;
use crate::tui_shell::App;

pub(super) fn on_lane_member_action(app: &mut App, value: String) {
    let Some(act) = parse_member_action(&value) else {
        prompts::open_lane_member_action_prompt(
            app,
            Some("error: choose add | remove".to_string()),
        );
        return;
    };

    if let Some(w) = app.lane_member_wizard.as_mut() {
        w.action = Some(act);
    }
    prompts::open_lane_member_lane_prompt(app, None);
}

pub(super) fn on_lane_member_lane(app: &mut App, value: String) {
    let lane = value.trim().to_string();
    if lane.is_empty() {
        prompts::open_lane_member_lane_prompt(app, Some("error: value required".to_string()));
        return;
    }
    if let Some(w) = app.lane_member_wizard.as_mut() {
        w.lane = Some(lane);
    }
    prompts::open_lane_member_handle_prompt(app, None);
}

pub(super) fn on_lane_member_handle(app: &mut App, value: String) {
    let handle = value.trim().to_string();
    if handle.is_empty() {
        prompts::open_lane_member_handle_prompt(app, Some("error: value required".to_string()));
        return;
    }
    if let Some(w) = app.lane_member_wizard.as_mut() {
        w.handle = Some(handle);
    }
    app.finish_lane_member_wizard();
}

fn parse_member_action(value: &str) -> Option<MemberAction> {
    match value.trim().to_lowercase().as_str() {
        "add" => Some(MemberAction::Add),
        "remove" | "rm" | "del" => Some(MemberAction::Remove),
        _ => None,
    }
}
