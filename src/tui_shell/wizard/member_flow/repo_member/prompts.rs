use crate::tui_shell::{App, TextInputAction};

pub(super) fn open_member_action_prompt(app: &mut App, error: Option<String>) {
    let mut lines = Vec::new();
    if let Some(e) = error {
        lines.push(e);
    } else {
        lines.push("add | remove".to_string());
    }
    app.open_text_input_modal(
        "Member",
        "action> ",
        TextInputAction::MemberAction,
        Some("add".to_string()),
        lines,
    );
}

pub(super) fn open_member_handle_prompt(app: &mut App, error: Option<String>) {
    let mut lines = Vec::new();
    if let Some(e) = error {
        lines.push(e);
    } else {
        lines.push("GitHub handle / user handle".to_string());
    }
    app.open_text_input_modal(
        "Member",
        "handle> ",
        TextInputAction::MemberHandle,
        None,
        lines,
    );
}

pub(super) fn open_member_role_prompt(
    app: &mut App,
    initial: Option<String>,
    error: Option<String>,
) {
    let mut lines = Vec::new();
    if let Some(e) = error {
        lines.push(e);
    } else {
        lines.push("Default: read".to_string());
    }
    app.open_text_input_modal(
        "Member",
        "role (read/publish)> ",
        TextInputAction::MemberRole,
        initial.or_else(|| Some("read".to_string())),
        lines,
    );
}
