use crate::tui_shell::{App, TextInputAction};

pub(super) fn open_publish_start_prompt(app: &mut App, scope: &str, gate: &str) {
    app.open_text_input_modal(
        "Publish",
        "publish> ",
        TextInputAction::PublishStart,
        None,
        vec![
            format!("Default: latest snap -> {}/{}", scope, gate),
            "Enter: publish now".to_string(),
            "Type `edit` to customize (snap/scope/gate/meta).".to_string(),
        ],
    );
}

pub(super) fn open_publish_snap_prompt(app: &mut App, edit_mode: bool) {
    let lines = if edit_mode {
        vec![
            "Optional: snap id (leave blank to publish latest).".to_string(),
            "Esc cancels.".to_string(),
        ]
    } else {
        vec!["Optional: snap id".to_string()]
    };
    app.open_text_input_modal(
        "Publish",
        "snap (blank=latest)> ",
        TextInputAction::PublishSnap,
        None,
        lines,
    );
}

pub(super) fn open_publish_custom_snap_prompt(app: &mut App) {
    app.open_text_input_modal(
        "Publish",
        "snap (blank=latest)> ",
        TextInputAction::PublishSnap,
        None,
        vec!["Optional: snap id".to_string()],
    );
}

pub(super) fn open_publish_scope_prompt(app: &mut App) {
    let initial = app.publish_wizard.as_ref().and_then(|w| w.scope.clone());
    app.open_text_input_modal(
        "Publish",
        "scope> ",
        TextInputAction::PublishScope,
        initial,
        vec!["Scope id (Enter keeps default).".to_string()],
    );
}

pub(super) fn open_publish_gate_prompt(app: &mut App) {
    let initial = app.publish_wizard.as_ref().and_then(|w| w.gate.clone());
    app.open_text_input_modal(
        "Publish",
        "gate> ",
        TextInputAction::PublishGate,
        initial,
        vec!["Gate id (Enter keeps default).".to_string()],
    );
}

pub(super) fn open_publish_meta_prompt(app: &mut App) {
    app.open_text_input_modal(
        "Publish",
        "metadata-only? (y/N)> ",
        TextInputAction::PublishMeta,
        Some("n".to_string()),
        vec!["If yes, publish metadata only (objects may be missing until later).".to_string()],
    );
}
