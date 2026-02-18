use super::*;

pub(super) fn apply_workflow_profile_set(app: &mut App, ws: &Workspace, value: String) {
    let profile = match value.trim().to_lowercase().as_str() {
        "software" => crate::model::WorkflowProfile::Software,
        "daw" => crate::model::WorkflowProfile::Daw,
        "game-assets" | "game_assets" | "gameassets" => crate::model::WorkflowProfile::GameAssets,
        _ => {
            app.push_error("expected: software | daw | game-assets".to_string());
            return;
        }
    };

    let mut cfg = match ws.store.read_config() {
        Ok(c) => c,
        Err(err) => {
            app.push_error(format!("read config: {:#}", err));
            return;
        }
    };
    cfg.workflow_profile = profile;
    if let Err(err) = ws.store.write_config(&cfg) {
        app.push_error(format!("write config: {:#}", err));
        return;
    }

    app.refresh_root_view();
    app.refresh_settings_view();
    app.push_output(vec![format!("workflow profile: {}", profile.as_str())]);
}
