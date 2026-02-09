use crate::tui_shell::TextInputAction;

pub(super) fn open_repo_prompt(app: &mut crate::tui_shell::App, default: String) {
    app.open_text_input_modal(
        "Bootstrap",
        "repo> ",
        TextInputAction::BootstrapRepo,
        Some(default),
        vec![
            "Repo id to use for the client config.".to_string(),
            "If it doesn't exist, the wizard will create it.".to_string(),
        ],
    );
}

pub(super) fn open_scope_prompt(app: &mut crate::tui_shell::App, default: String) {
    app.open_text_input_modal(
        "Bootstrap",
        "scope> ",
        TextInputAction::BootstrapScope,
        Some(default),
        vec!["Default scope for remote operations.".to_string()],
    );
}

pub(super) fn open_gate_prompt(app: &mut crate::tui_shell::App, default: String) {
    app.open_text_input_modal(
        "Bootstrap",
        "gate> ",
        TextInputAction::BootstrapGate,
        Some(default),
        vec!["Default gate for remote operations.".to_string()],
    );
}

pub(super) fn open_token_prompt(app: &mut crate::tui_shell::App) {
    app.open_text_input_modal(
        "Bootstrap",
        "bootstrap token> ",
        TextInputAction::BootstrapToken,
        None,
        vec![
            "One-time bootstrap token (the same value passed to converge-server --bootstrap-token)."
                .to_string(),
            "Generate one: openssl rand -hex 32".to_string(),
        ],
    );
}

pub(super) fn open_handle_prompt(app: &mut crate::tui_shell::App) {
    app.open_text_input_modal(
        "Bootstrap",
        "admin handle> ",
        TextInputAction::BootstrapHandle,
        Some("admin".to_string()),
        vec![
            "Admin handle to create (one-time).".to_string(),
            "Response includes a plaintext admin token; it will be stored in .converge/state.json"
                .to_string(),
        ],
    );
}

pub(super) fn open_display_name_prompt(app: &mut crate::tui_shell::App) {
    app.open_text_input_modal(
        "Bootstrap",
        "display name (optional)> ",
        TextInputAction::BootstrapDisplayName,
        None,
        vec!["Optional display name (leave blank to skip).".to_string()],
    );
}
