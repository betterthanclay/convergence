use super::*;

pub(super) fn is_direct_command_action(action: &TextInputAction) -> bool {
    matches!(
        action,
        TextInputAction::ReleaseBundleId
            | TextInputAction::PromoteToGate
            | TextInputAction::PromoteBundleId
            | TextInputAction::PinBundleId
            | TextInputAction::PinAction
            | TextInputAction::ApproveBundleId
            | TextInputAction::SuperpositionsBundleId
    )
}

pub(super) fn apply_direct_command_text_input(
    app: &mut App,
    action: TextInputAction,
    value: String,
) {
    match action {
        TextInputAction::ReleaseBundleId => {
            let id = value.trim().to_string();
            if id.is_empty() {
                app.push_error("missing bundle id".to_string());
                return;
            }
            app.start_release_wizard(id);
        }
        TextInputAction::PromoteToGate => {
            app.continue_promote_wizard(value);
        }
        TextInputAction::PromoteBundleId => {
            let id = value.trim().to_string();
            if id.is_empty() {
                app.push_error("missing bundle id".to_string());
                return;
            }
            app.cmd_promote(&["--bundle-id".to_string(), id]);
        }
        TextInputAction::PinBundleId => {
            let id = value.trim().to_string();
            if id.is_empty() {
                app.push_error("missing bundle id".to_string());
                return;
            }
            if let Some(wizard) = app.pin_wizard.as_mut() {
                wizard.bundle_id = Some(id);
            }
            app.open_text_input_modal(
                "Pin",
                "action (pin/unpin)> ",
                TextInputAction::PinAction,
                Some("pin".to_string()),
                vec!["Choose pin or unpin".to_string()],
            );
        }
        TextInputAction::PinAction => app.finish_pin_wizard(value),
        TextInputAction::ApproveBundleId => {
            let id = value.trim().to_string();
            if id.is_empty() {
                app.push_error("missing bundle id".to_string());
                return;
            }
            app.cmd_approve(&["--bundle-id".to_string(), id]);
        }
        TextInputAction::SuperpositionsBundleId => {
            let id = value.trim().to_string();
            if id.is_empty() {
                app.push_error("missing bundle id".to_string());
                return;
            }
            app.cmd_superpositions(&["--bundle-id".to_string(), id]);
        }
        _ => app.push_error("unexpected direct command text input action".to_string()),
    }
}
