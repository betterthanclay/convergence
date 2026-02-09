pub(super) fn on_fetch_user(app: &mut crate::tui_shell::App, value: String) {
    let v = value.trim().to_string();
    if let Some(w) = app.fetch_wizard.as_mut() {
        w.user = if v.is_empty() { None } else { Some(v) };
    }
    app.finish_fetch_wizard();
}

pub(super) fn on_fetch_options(app: &mut crate::tui_shell::App, value: String) {
    if let Some(w) = app.fetch_wizard.as_mut() {
        let v = value.trim().to_string();
        w.options = if v.is_empty() { None } else { Some(v) };
    }
    app.finish_fetch_wizard();
}
