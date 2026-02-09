use super::prompts;
use super::*;

pub(super) fn on_fetch_id(app: &mut crate::tui_shell::App, value: String) {
    let kind = app.fetch_wizard.as_ref().and_then(|w| w.kind);
    let Some(kind) = kind else {
        app.start_fetch_wizard();
        return;
    };

    let id = value.trim().to_string();
    if id.is_empty() && kind != FetchKind::Snap {
        prompts::open_fetch_id_prompt(app, kind, Some("error: value required".to_string()));
        return;
    }

    if let Some(w) = app.fetch_wizard.as_mut() {
        w.id = if id.is_empty() { None } else { Some(id) };
    }

    match kind {
        FetchKind::Lane => prompts::open_lane_user_prompt(app),
        FetchKind::Bundle | FetchKind::Release => prompts::open_fetch_options_prompt(app),
        FetchKind::Snap => app.finish_fetch_wizard(),
    }
}
