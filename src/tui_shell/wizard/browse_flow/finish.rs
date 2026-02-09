use super::*;
use crate::tui_shell::App;

pub(super) fn finish_browse_wizard(app: &mut App) {
    let Some(w) = app.browse_wizard.clone() else {
        app.push_error("browse wizard not active".to_string());
        return;
    };
    app.browse_wizard = None;

    match w.target {
        BrowseTarget::Inbox => app.open_inbox_view(w.scope, w.gate, w.filter, w.limit),
        BrowseTarget::Bundles => app.open_bundles_view(w.scope, w.gate, w.filter, w.limit),
    }
}
