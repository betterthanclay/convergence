use super::super::super::*;

pub(super) fn handle_escape(app: &mut App) {
    if !app.input.buf.is_empty() {
        app.input.clear();
        app.recompute_suggestions();
    } else if app.mode() != UiMode::Root {
        app.pop_mode();
        app.push_output(vec![format!("back to {:?}", app.mode())]);
    } else {
        app.quit = true;
    }
}

pub(super) fn handle_tab(app: &mut App) {
    if app.input.buf.is_empty() {
        if app.root_ctx == RootContext::Local && app.mode() == UiMode::Root {
            app.switch_to_remote_root();
            app.push_output(vec!["switched to remote context".to_string()]);
        } else if app.root_ctx == RootContext::Remote {
            app.switch_to_local_root();
            app.push_output(vec!["switched to local context".to_string()]);
        }
    } else if !app.suggestions.is_empty() {
        app.apply_selected_suggestion();
    }
}

pub(super) fn handle_enter(app: &mut App) {
    if app.input.buf.is_empty() {
        app.run_default_action();
        return;
    }

    if !app.suggestions.is_empty() {
        let sel = app
            .suggestion_selected
            .min(app.suggestions.len().saturating_sub(1));
        let cmd = app.suggestions[sel].name;

        let raw = app.input.buf.trim_start_matches('/').trim_start();
        let first = raw.split_whitespace().next().unwrap_or("");
        if first != cmd {
            app.apply_selected_suggestion();
        }
    }
    app.run_current_input();
}
