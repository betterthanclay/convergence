use super::super::super::TextInputAction;
use super::super::FetchKind;
use crate::tui_shell::App;

pub(super) fn open_fetch_kind_prompt(app: &mut App, error: Option<String>) {
    let mut lines = Vec::new();
    if let Some(e) = error {
        lines.push(e);
        lines.push(String::new());
    }
    lines.push("What to fetch?".to_string());

    app.open_text_input_modal(
        "Fetch",
        "what> ",
        TextInputAction::FetchKind,
        Some("snap".to_string()),
        lines,
    );
}

pub(super) fn open_fetch_id_prompt(app: &mut App, kind: FetchKind, error: Option<String>) {
    let (prompt, initial, mut lines) = match kind {
        FetchKind::Snap => (
            "snap id (blank=all)> ",
            None,
            vec!["Optional: leave blank to fetch all publications.".to_string()],
        ),
        FetchKind::Bundle => ("bundle id> ", None, vec!["Paste bundle id".to_string()]),
        FetchKind::Release => (
            "channel> ",
            None,
            vec!["Release channel name (example: main)".to_string()],
        ),
        FetchKind::Lane => (
            "lane id> ",
            Some("default".to_string()),
            vec!["Lane id (example: default)".to_string()],
        ),
    };
    if let Some(e) = error {
        lines.insert(0, e);
    }
    app.open_text_input_modal("Fetch", prompt, TextInputAction::FetchId, initial, lines);
}

pub(super) fn open_lane_user_prompt(app: &mut App) {
    app.open_text_input_modal(
        "Fetch",
        "user (blank=all)> ",
        TextInputAction::FetchUser,
        None,
        vec!["Optional: filter by user handle".to_string()],
    );
}

pub(super) fn open_fetch_options_prompt(app: &mut App) {
    app.open_text_input_modal(
        "Fetch",
        "options> ",
        TextInputAction::FetchOptions,
        None,
        vec![
            "Optional:".to_string(),
            "- empty: fetch only".to_string(),
            "- restore: also materialize into a directory".to_string(),
            "- into <dir>: choose directory (implies restore)".to_string(),
            "- force: overwrite files when restoring".to_string(),
        ],
    );
}
