use super::*;

pub(super) fn set(app: &mut App) {
    let initial = app
        .current_view::<SettingsView>()
        .and_then(|v| v.snapshot)
        .map(|s| s.workflow_profile.as_str().to_string())
        .or_else(|| Some("software".to_string()));
    app.open_text_input_modal(
        "Workflow Profile",
        "profile> ",
        TextInputAction::WorkflowProfileSet,
        initial,
        vec![
            "Choose one: software | daw | game-assets".to_string(),
            "This only adjusts guidance text and hints.".to_string(),
        ],
    );
}
