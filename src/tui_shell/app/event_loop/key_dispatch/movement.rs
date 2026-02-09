use super::super::super::*;

pub(super) fn handle_up(app: &mut App) {
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

pub(super) fn handle_down(app: &mut App) {
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

pub(super) fn handle_left(app: &mut App) {
    if app.input.buf.is_empty() {
        app.rotate_hint(-1);
    } else {
        app.input.move_left();
    }
}

pub(super) fn handle_right(app: &mut App) {
    if app.input.buf.is_empty() {
        app.rotate_hint(1);
    } else {
        app.input.move_right();
    }
}
