use super::*;

pub(super) fn selected_inbox_publication_id(app: &mut App) -> Option<String> {
    let Some(v) = app.current_view::<InboxView>() else {
        app.push_error("not in inbox mode".to_string());
        return None;
    };
    if v.items.is_empty() {
        app.push_error("(no selection)".to_string());
        return None;
    }
    let idx = v.selected.min(v.items.len().saturating_sub(1));
    Some(v.items[idx].id.clone())
}

pub(super) fn selected_inbox_snap_id(app: &mut App) -> Option<String> {
    let Some(v) = app.current_view::<InboxView>() else {
        app.push_error("not in inbox mode".to_string());
        return None;
    };
    if v.items.is_empty() {
        app.push_error("(no selection)".to_string());
        return None;
    }
    let idx = v.selected.min(v.items.len().saturating_sub(1));
    Some(v.items[idx].snap_id.clone())
}

pub(super) fn selected_bundle_id(app: &mut App) -> Option<String> {
    let Some(v) = app.current_view::<BundlesView>() else {
        app.push_error("not in bundles mode".to_string());
        return None;
    };
    if v.items.is_empty() {
        app.push_error("(no selection)".to_string());
        return None;
    }
    let idx = v.selected.min(v.items.len().saturating_sub(1));
    Some(v.items[idx].id.clone())
}
