use super::super::super::*;

pub(super) fn hint_commands_raw(app: &App) -> Vec<String> {
    match app.mode() {
        UiMode::Root => root_mode_hints(app),
        UiMode::Snaps => snaps_mode_hints(app),
        UiMode::Inbox => vec!["bundle".to_string(), "fetch".to_string()],
        UiMode::Releases => vec!["fetch".to_string(), "back".to_string()],
        UiMode::Lanes => vec!["fetch".to_string(), "back".to_string()],
        UiMode::Bundles => bundles_mode_hints(app),
        UiMode::Superpositions => superpositions_mode_hints(app),
        UiMode::GateGraph => Vec::new(),
        UiMode::Settings => settings_mode_hints(app),
    }
}

fn root_mode_hints(app: &App) -> Vec<String> {
    match app.root_ctx {
        RootContext::Local => {
            if app.workspace.is_none() {
                if app
                    .workspace_err
                    .as_deref()
                    .is_some_and(|e| e.contains("No .converge directory found"))
                {
                    return vec!["init".to_string()];
                }
                return Vec::new();
            }

            let mut changes = 0usize;
            if let Some(v) = app.current_view::<RootView>() {
                changes = v.change_summary.added
                    + v.change_summary.modified
                    + v.change_summary.deleted
                    + v.change_summary.renamed;
            }
            if changes > 0 {
                return vec!["snap".to_string(), "history".to_string()];
            }

            if app.remote_configured {
                let latest = app.latest_snap_id.clone();
                let synced = app.lane_last_synced.get("default").cloned();
                if latest.is_some() && latest != synced {
                    return vec!["sync".to_string(), "history".to_string()];
                }
                if latest.is_some() && latest != app.last_published_snap_id {
                    return vec!["publish".to_string(), "history".to_string()];
                }
            }

            vec!["history".to_string()]
        }
        RootContext::Remote => {
            if !app.remote_configured || app.remote_identity.is_none() {
                vec!["login".to_string(), "bootstrap".to_string()]
            } else if app.remote_repo_missing() {
                vec!["create-repo".to_string()]
            } else {
                vec!["inbox".to_string(), "releases".to_string()]
            }
        }
    }
}

fn snaps_mode_hints(app: &App) -> Vec<String> {
    let Some(v) = app.current_view::<SnapsView>() else {
        return Vec::new();
    };
    if v.selected_is_pending() {
        vec!["snap".to_string(), "revert".to_string()]
    } else if v.selected_is_clean() {
        vec!["unsnap".to_string()]
    } else {
        vec!["restore".to_string(), "msg".to_string()]
    }
}

fn bundles_mode_hints(app: &App) -> Vec<String> {
    let Some(v) = app.current_view::<BundlesView>() else {
        return Vec::new();
    };
    if v.items.is_empty() {
        return vec!["back".to_string()];
    }
    let idx = v.selected.min(v.items.len().saturating_sub(1));
    let b = &v.items[idx];

    if b.reasons.iter().any(|r| r == "superpositions_present") {
        return vec!["superpositions".to_string(), "back".to_string()];
    }
    if b.reasons.iter().any(|r| r == "approvals_missing") {
        return vec!["approve".to_string(), "back".to_string()];
    }
    if b.promotable {
        return vec!["promote".to_string(), "back".to_string()];
    }

    vec!["back".to_string()]
}

fn superpositions_mode_hints(app: &App) -> Vec<String> {
    let Some(v) = app.current_view::<SuperpositionsView>() else {
        return Vec::new();
    };
    let missing = v
        .validation
        .as_ref()
        .map(|x| !x.missing.is_empty())
        .unwrap_or(false);
    if missing {
        vec!["next-missing".to_string(), "pick".to_string()]
    } else {
        vec!["apply".to_string(), "back".to_string()]
    }
}

fn settings_mode_hints(app: &App) -> Vec<String> {
    let Some(v) = app.current_view::<SettingsView>() else {
        return vec!["back".to_string()];
    };
    match v.selected_kind() {
        None => vec!["back".to_string()],
        Some(_) => vec!["do".to_string(), "back".to_string()],
    }
}
