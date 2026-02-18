use super::*;
use crate::tui_shell::App;

fn compact_query(scope: &str, gate: &str, filter: &Option<String>, limit: Option<usize>) -> String {
    let filter = filter.clone().unwrap_or_else(|| "none".to_string());
    let limit = limit
        .map(|n| n.to_string())
        .unwrap_or_else(|| "none".to_string());
    format!(
        "scope={} gate={} filter={} limit={}",
        scope, gate, filter, limit
    )
}

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

    let initial = app
        .browse_wizard
        .as_ref()
        .map(|w| compact_query(&w.scope, &w.gate, &w.filter, w.limit));
    app.open_text_input_modal(
        "Browse",
        "query> ",
        TextInputAction::BrowseQuery,
        initial,
        vec![
            "Edit fields in one line.".to_string(),
            "Format: scope=<id> gate=<id> filter=<q|none> limit=<n|none>".to_string(),
            "Positional also works: <scope> <gate> [filter] [limit]".to_string(),
            "Blank keeps current values.".to_string(),
        ],
    );
}
