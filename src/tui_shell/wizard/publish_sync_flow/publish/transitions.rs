use crate::tui_shell::App;

use super::prompts;

pub(super) fn on_publish_start(app: &mut App, value: String) {
    let v = value.trim().to_string();
    if v.is_empty() {
        app.publish_wizard = None;
        app.cmd_publish_impl(&[]);
        return;
    }

    let v_lc = v.to_lowercase();
    if matches!(v_lc.as_str(), "edit" | "prompt" | "custom") {
        prompts::open_publish_custom_snap_prompt(app);
        return;
    }

    if let Some(w) = app.publish_wizard.as_mut() {
        w.snap = Some(v);
    }
    prompts::open_publish_scope_prompt(app);
}

pub(super) fn on_publish_snap(app: &mut App, value: String) {
    let v = value.trim().to_string();
    if let Some(w) = app.publish_wizard.as_mut() {
        w.snap = if v.is_empty() { None } else { Some(v) };
    }
    prompts::open_publish_scope_prompt(app);
}

pub(super) fn on_publish_scope(app: &mut App, value: String) {
    let v = value.trim().to_string();
    if let Some(w) = app.publish_wizard.as_mut() {
        w.scope = if v.is_empty() { None } else { Some(v) };
    }
    prompts::open_publish_gate_prompt(app);
}

pub(super) fn on_publish_gate(app: &mut App, value: String) {
    let v = value.trim().to_string();
    if let Some(w) = app.publish_wizard.as_mut() {
        w.gate = if v.is_empty() { None } else { Some(v) };
    }
    prompts::open_publish_meta_prompt(app);
}

pub(super) fn on_publish_meta(app: &mut App, value: String) {
    let v = value.trim().to_lowercase();
    let meta = matches!(v.as_str(), "y" | "yes" | "true" | "1");
    if let Some(w) = app.publish_wizard.as_mut() {
        w.meta = meta;
    }
    app.finish_publish_wizard();
}
