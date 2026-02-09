use super::prompts;
use super::*;

pub(super) fn on_fetch_kind(app: &mut crate::tui_shell::App, value: String) {
    let v = value.trim().to_lowercase();
    let v = if v.is_empty() { "snap".to_string() } else { v };
    let kind = match v.as_str() {
        "snap" | "snaps" => Some(FetchKind::Snap),
        "bundle" | "bundles" => Some(FetchKind::Bundle),
        "release" | "releases" => Some(FetchKind::Release),
        "lane" | "lanes" => Some(FetchKind::Lane),
        _ => None,
    };

    let Some(kind) = kind else {
        prompts::open_fetch_kind_prompt(
            app,
            Some("error: choose snap | bundle | release | lane".to_string()),
        );
        return;
    };

    if let Some(w) = app.fetch_wizard.as_mut() {
        w.kind = Some(kind);
    }

    prompts::open_fetch_id_prompt(app, kind, None);
}
