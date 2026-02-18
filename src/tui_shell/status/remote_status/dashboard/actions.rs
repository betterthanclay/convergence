use super::super::DashboardData;

pub(super) fn recommended_actions(data: &DashboardData) -> Vec<String> {
    let mut actions = Vec::new();
    let blocked_supers = format!(
        "resolve superpositions ({}) [bundles -> superpositions]",
        data.blocked_superpositions
    );
    let blocked_approvals = format!(
        "collect approvals ({}) [bundles -> approve]",
        data.blocked_approvals
    );
    let promote = format!(
        "promote bundles ({}) [bundles -> promote]",
        data.bundles_promotable
    );
    let inbox = format!("open inbox ({} pending) [inbox]", data.inbox_pending);
    let fetch_missing = format!("fetch missing snaps ({}) [fetch]", data.inbox_missing_local);

    match data.workflow_profile {
        crate::model::WorkflowProfile::GameAssets => {
            if data.blocked_superpositions > 0 {
                actions.push(blocked_supers);
            }
            if data.blocked_approvals > 0 {
                actions.push(blocked_approvals);
            }
            if data.bundles_promotable > 0 {
                actions.push(promote);
            }
            if data.inbox_pending > 0 {
                actions.push(inbox);
            }
            if data.inbox_missing_local > 0 {
                actions.push(fetch_missing);
            }
        }
        crate::model::WorkflowProfile::Daw => {
            if data.inbox_pending > 0 {
                actions.push(inbox);
            }
            if data.bundles_promotable > 0 {
                actions.push(promote);
            }
            if data.blocked_approvals > 0 {
                actions.push(blocked_approvals);
            }
            if data.blocked_superpositions > 0 {
                actions.push(blocked_supers);
            }
            if data.inbox_missing_local > 0 {
                actions.push(fetch_missing);
            }
        }
        crate::model::WorkflowProfile::Software => {
            if data.inbox_pending > 0 {
                actions.push(inbox);
            }
            if data.inbox_missing_local > 0 {
                actions.push(fetch_missing);
            }
            if data.bundles_promotable > 0 {
                actions.push(promote);
            }
            if data.blocked_superpositions > 0 {
                actions.push(blocked_supers);
            }
            if data.blocked_approvals > 0 {
                actions.push(blocked_approvals);
            }
        }
    }
    actions.into_iter().take(4).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::WorkflowProfile;

    fn with_profile(profile: WorkflowProfile) -> DashboardData {
        let mut d = super::super::new_dashboard_data();
        d.workflow_profile = profile;
        d.inbox_pending = 3;
        d.inbox_missing_local = 2;
        d.bundles_promotable = 1;
        d.blocked_superpositions = 4;
        d.blocked_approvals = 5;
        d
    }

    #[test]
    fn game_assets_prioritizes_blockers_first() {
        let d = with_profile(WorkflowProfile::GameAssets);
        let actions = recommended_actions(&d);
        assert!(actions[0].starts_with("resolve superpositions"));
        assert!(actions[1].starts_with("collect approvals"));
    }

    #[test]
    fn daw_prioritizes_inbox_first() {
        let d = with_profile(WorkflowProfile::Daw);
        let actions = recommended_actions(&d);
        assert!(actions[0].starts_with("open inbox"));
        assert!(actions[1].starts_with("promote bundles"));
    }
}
