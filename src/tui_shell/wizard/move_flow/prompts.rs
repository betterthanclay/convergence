use super::super::super::{App, TextInputAction};

pub(super) fn open_from_error_prompt(app: &mut App, raw: String, error: String) {
    app.open_text_input_modal(
        "Move",
        "from (glob)> ",
        TextInputAction::MoveFrom,
        Some(raw),
        vec![
            format!("error: {}", error),
            String::new(),
            "Enter a glob to find the source path.".to_string(),
        ],
    );
}

pub(super) fn open_no_matches_prompt(app: &mut App, raw: String) {
    app.open_text_input_modal(
        "Move",
        "from (glob)> ",
        TextInputAction::MoveFrom,
        Some(raw),
        vec![
            "error: no matches".to_string(),
            String::new(),
            "Try a more specific glob.".to_string(),
        ],
    );
}

pub(super) fn open_to_prompt(app: &mut App, from: String) {
    app.open_text_input_modal(
        "Move",
        "to> ",
        TextInputAction::MoveTo,
        Some(from.clone()),
        vec![
            format!("from: {}", from),
            "Edit the destination path (relative to workspace root).".to_string(),
        ],
    );
}

pub(super) fn open_candidates_prompt(app: &mut App, raw: String, matches: &[String]) {
    let mut lines = Vec::new();
    lines.push(format!("matches: {}", matches.len()));
    lines.push("Enter a number to pick, or refine the glob.".to_string());
    lines.push(String::new());

    let limit = 20usize;
    for (i, p) in matches.iter().take(limit).enumerate() {
        lines.push(format!("{:>2}. {}", i + 1, p));
    }
    if matches.len() > limit {
        lines.push(format!("… and {} more", matches.len() - limit));
    }

    app.open_text_input_modal(
        "Move",
        "from (glob or #)> ",
        TextInputAction::MoveFrom,
        Some(raw),
        lines,
    );
}

pub(super) fn open_to_retry_prompt(app: &mut App, from: String, to: String, error: String) {
    app.open_text_input_modal(
        "Move",
        "to> ",
        TextInputAction::MoveTo,
        Some(to),
        vec![
            format!("from: {}", from),
            format!("error: {}", error),
            "Edit destination and try again.".to_string(),
        ],
    );
}
