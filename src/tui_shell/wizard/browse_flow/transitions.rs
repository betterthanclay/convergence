use super::*;
use crate::tui_shell::App;

fn open_gate_prompt(app: &mut App) {
    let initial = app.browse_wizard.as_ref().map(|w| w.gate.clone());
    app.open_text_input_modal(
        "Browse",
        "gate> ",
        TextInputAction::BrowseGate,
        initial,
        vec!["Gate id (Enter keeps current).".to_string()],
    );
}

fn open_filter_prompt(app: &mut App) {
    let initial = app.browse_wizard.as_ref().and_then(|w| w.filter.clone());
    app.open_text_input_modal(
        "Browse",
        "filter (blank=none)> ",
        TextInputAction::BrowseFilter,
        initial,
        vec!["Optional filter query".to_string()],
    );
}

fn open_limit_prompt(app: &mut App, initial: Option<String>, help: Vec<String>) {
    app.open_text_input_modal(
        "Browse",
        "limit (blank=none)> ",
        TextInputAction::BrowseLimit,
        initial,
        help,
    );
}

pub(super) fn continue_browse_wizard(app: &mut App, action: TextInputAction, value: String) {
    if app.browse_wizard.is_none() {
        app.push_error("browse wizard not active".to_string());
        return;
    }

    match action {
        TextInputAction::BrowseScope => {
            let v = value.trim().to_string();
            if let Some(w) = app.browse_wizard.as_mut()
                && !v.is_empty()
            {
                w.scope = v;
            }
            open_gate_prompt(app);
        }
        TextInputAction::BrowseGate => {
            let v = value.trim().to_string();
            if let Some(w) = app.browse_wizard.as_mut()
                && !v.is_empty()
            {
                w.gate = v;
            }
            open_filter_prompt(app);
        }
        TextInputAction::BrowseFilter => {
            let v = value.trim().to_string();
            if let Some(w) = app.browse_wizard.as_mut() {
                w.filter = if v.is_empty() { None } else { Some(v) };
            }
            let initial = app
                .browse_wizard
                .as_ref()
                .and_then(|w| w.limit)
                .map(|n| n.to_string());
            open_limit_prompt(app, initial, vec!["Optional limit".to_string()]);
        }
        TextInputAction::BrowseLimit => {
            let v = value.trim().to_string();
            let limit = if v.is_empty() {
                None
            } else {
                match v.parse::<usize>() {
                    Ok(n) => Some(n),
                    Err(_) => {
                        open_limit_prompt(app, Some(v), vec!["error: invalid number".to_string()]);
                        return;
                    }
                }
            };
            if let Some(w) = app.browse_wizard.as_mut() {
                w.limit = limit;
            }
            app.finish_browse_wizard();
        }
        _ => {
            app.push_error("unexpected browse wizard input".to_string());
        }
    }
}
