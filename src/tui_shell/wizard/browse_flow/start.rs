use super::*;
use crate::tui_shell::App;

pub(super) fn start_browse_wizard(app: &mut App, target: BrowseTarget) {
    let cfg = match app.remote_config() {
        Some(c) => c,
        None => {
            app.start_login_wizard();
            return;
        }
    };

    let (scope, gate, filter, limit) = match target {
        BrowseTarget::Inbox => app
            .current_view::<InboxView>()
            .map(|v| (v.scope.clone(), v.gate.clone(), v.filter.clone(), v.limit))
            .unwrap_or((cfg.scope.clone(), cfg.gate.clone(), None, None)),
        BrowseTarget::Bundles => app
            .current_view::<BundlesView>()
            .map(|v| (v.scope.clone(), v.gate.clone(), v.filter.clone(), v.limit))
            .unwrap_or((cfg.scope.clone(), cfg.gate.clone(), None, None)),
    };

    app.browse_wizard = Some(BrowseWizard {
        target,
        scope,
        gate,
        filter,
        limit,
    });

    let initial = app.browse_wizard.as_ref().map(|w| w.scope.clone());
    app.open_text_input_modal(
        "Browse",
        "scope> ",
        TextInputAction::BrowseScope,
        initial,
        vec!["Scope id (Enter keeps current).".to_string()],
    );
}
