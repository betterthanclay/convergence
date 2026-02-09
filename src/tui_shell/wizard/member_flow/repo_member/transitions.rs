use super::MemberAction;
use super::prompts;
use crate::tui_shell::App;

pub(super) fn on_member_action(app: &mut App, value: String) {
    let Some(act) = parse_member_action(&value) else {
        prompts::open_member_action_prompt(app, Some("error: choose add | remove".to_string()));
        return;
    };

    if let Some(w) = app.member_wizard.as_mut() {
        w.action = Some(act);
    }
    prompts::open_member_handle_prompt(app, None);
}

pub(super) fn on_member_handle(app: &mut App, value: String) {
    let handle = value.trim().to_string();
    if handle.is_empty() {
        prompts::open_member_handle_prompt(app, Some("error: value required".to_string()));
        return;
    }

    let act = app.member_wizard.as_ref().and_then(|w| w.action);
    if let Some(w) = app.member_wizard.as_mut() {
        w.handle = Some(handle);
    }

    match act {
        Some(MemberAction::Add) => {
            prompts::open_member_role_prompt(app, Some("read".to_string()), None)
        }
        Some(MemberAction::Remove) => app.finish_member_wizard(),
        None => app.start_member_wizard(None),
    }
}

pub(super) fn on_member_role(app: &mut App, value: String) {
    let role = normalize_role(&value);
    if role != "read" && role != "publish" {
        prompts::open_member_role_prompt(
            app,
            Some(role),
            Some("error: role must be read or publish".to_string()),
        );
        return;
    }

    if let Some(w) = app.member_wizard.as_mut() {
        w.role = role;
    }
    app.finish_member_wizard();
}

fn parse_member_action(value: &str) -> Option<MemberAction> {
    match value.trim().to_lowercase().as_str() {
        "add" => Some(MemberAction::Add),
        "remove" | "rm" | "del" => Some(MemberAction::Remove),
        _ => None,
    }
}

fn normalize_role(value: &str) -> String {
    let role = value.trim().to_lowercase();
    if role.is_empty() {
        "read".to_string()
    } else {
        role
    }
}
