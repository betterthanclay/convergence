use super::super::DashboardData;

pub(super) fn recommended_actions(data: &DashboardData) -> Vec<String> {
    let mut actions = Vec::new();
    if data.inbox_pending > 0 {
        actions.push(format!("open inbox ({} pending)", data.inbox_pending));
    }
    if data.inbox_missing_local > 0 {
        actions.push(format!(
            "fetch missing snaps ({})",
            data.inbox_missing_local
        ));
    }
    if data.bundles_promotable > 0 {
        actions.push(format!("promote bundles ({})", data.bundles_promotable));
    }
    if data.blocked_superpositions > 0 {
        actions.push(format!(
            "resolve superpositions ({})",
            data.blocked_superpositions
        ));
    }
    if data.blocked_approvals > 0 {
        actions.push(format!("collect approvals ({})", data.blocked_approvals));
    }
    actions.into_iter().take(4).collect()
}
