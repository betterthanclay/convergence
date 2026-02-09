use super::super::super::*;

pub(super) fn hint_key(app: &App) -> usize {
    match (app.mode(), app.root_ctx) {
        (UiMode::Root, RootContext::Local) => 0,
        (UiMode::Root, RootContext::Remote) => 1,
        (UiMode::Snaps, _) => 2,
        (UiMode::Inbox, _) => 3,
        (UiMode::Bundles, _) => 4,
        (UiMode::Releases, _) => 5,
        (UiMode::Lanes, _) => 6,
        (UiMode::Superpositions, _) => 7,
        (UiMode::GateGraph, _) => 8,
        (UiMode::Settings, _) => 9,
    }
}

pub(super) fn rotate_hint(app: &mut App, dir: i32) {
    if !app.input.buf.is_empty() || app.modal.is_some() {
        return;
    }

    let n = super::mode_hints::hint_commands_raw(app).len();
    if n <= 1 {
        app.hint_rotation[hint_key(app)] = 0;
        return;
    }

    let key = hint_key(app);

    if dir > 0 {
        app.hint_rotation[key] = (app.hint_rotation[key] + 1) % n;
    } else if dir < 0 {
        app.hint_rotation[key] = (app.hint_rotation[key] + n - 1) % n;
    }
}
