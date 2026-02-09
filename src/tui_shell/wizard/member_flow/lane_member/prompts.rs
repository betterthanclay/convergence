use crate::tui_shell::{App, TextInputAction};

pub(super) fn open_lane_member_action_prompt(app: &mut App, error: Option<String>) {
    let mut lines = Vec::new();
    if let Some(e) = error {
        lines.push(e);
    } else {
        lines.push("add | remove".to_string());
    }
    app.open_text_input_modal(
        "Lane Member",
        "action> ",
        TextInputAction::LaneMemberAction,
        Some("add".to_string()),
        lines,
    );
}

pub(super) fn open_lane_member_lane_prompt(app: &mut App, error: Option<String>) {
    let mut lines = Vec::new();
    if let Some(e) = error {
        lines.push(e);
    } else {
        lines.push("Lane id".to_string());
    }
    app.open_text_input_modal(
        "Lane Member",
        "lane> ",
        TextInputAction::LaneMemberLane,
        Some("default".to_string()),
        lines,
    );
}

pub(super) fn open_lane_member_handle_prompt(app: &mut App, error: Option<String>) {
    let mut lines = Vec::new();
    if let Some(e) = error {
        lines.push(e);
    } else {
        lines.push("User handle".to_string());
    }
    app.open_text_input_modal(
        "Lane Member",
        "handle> ",
        TextInputAction::LaneMemberHandle,
        None,
        lines,
    );
}
