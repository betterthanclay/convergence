use super::super::*;

pub(super) fn handle_key(app: &mut App, key: KeyEvent) {
    if app.modal.is_some() {
        modal::handle_modal_key(app, key);
        return;
    }

    match key.code {
        KeyCode::Char('q') => app.quit = true,
        KeyCode::Esc => handle_escape(app),
        KeyCode::Tab => handle_tab(app),
        KeyCode::Enter => handle_enter(app),
        KeyCode::Up => handle_up(app),
        KeyCode::Down => handle_down(app),
        KeyCode::Left => handle_left(app),
        KeyCode::Right => handle_right(app),
        KeyCode::Backspace => {
            app.input.backspace();
            app.recompute_suggestions();
        }
        KeyCode::Delete => {
            app.input.delete();
            app.recompute_suggestions();
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input.clear();
            app.recompute_suggestions();
        }
        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input.history_up();
            app.recompute_suggestions();
        }
        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input.history_down();
            app.recompute_suggestions();
        }
        KeyCode::Char(c)
            if key.modifiers.contains(KeyModifiers::ALT) && app.input.buf.is_empty() =>
        {
            handle_alt_shortcut(app, c);
        }
        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input.insert_char(c);
            app.recompute_suggestions();
        }
        _ => {}
    }
}

fn handle_escape(app: &mut App) {
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

fn handle_tab(app: &mut App) {
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

fn handle_enter(app: &mut App) {
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

fn handle_up(app: &mut App) {
    if app.input.buf.is_empty() {
        app.view_mut().move_up();
        return;
    }
    if !app.suggestions.is_empty() {
        let n = app.suggestions.len();
        if n > 0 {
            app.suggestion_selected = (app.suggestion_selected + n - 1) % n;
        }
        return;
    }
    app.input.history_up();
    app.recompute_suggestions();
}

fn handle_down(app: &mut App) {
    if app.input.buf.is_empty() {
        app.view_mut().move_down();
        return;
    }
    if !app.suggestions.is_empty() {
        let n = app.suggestions.len();
        if n > 0 {
            app.suggestion_selected = (app.suggestion_selected + 1) % n;
        }
        return;
    }
    app.input.history_down();
    app.recompute_suggestions();
}

fn handle_left(app: &mut App) {
    if app.input.buf.is_empty() {
        app.rotate_hint(-1);
    } else {
        app.input.move_left();
    }
}

fn handle_right(app: &mut App) {
    if app.input.buf.is_empty() {
        app.rotate_hint(1);
    } else {
        app.input.move_right();
    }
}

fn handle_alt_shortcut(app: &mut App, c: char) {
    if app.mode() != UiMode::Superpositions {
        return;
    }

    if c.is_ascii_digit() {
        let n = c.to_digit(10).unwrap_or(0) as usize;
        // Alt+0 clears; Alt+1..9 selects variant.
        if n == 0 {
            superpositions_nav::superpositions_clear_decision(app);
        } else {
            superpositions_nav::superpositions_pick_variant(app, n - 1);
        }
    }

    if c == 'f' {
        superpositions_nav::superpositions_jump_next_invalid(app);
    }

    if c == 'n' {
        superpositions_nav::superpositions_jump_next_missing(app);
    }
}
