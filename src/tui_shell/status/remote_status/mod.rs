use crate::model::WorkflowProfile;
use crate::remote::RemoteClient;
use crate::tui_shell::{RenderCtx, fmt_ts_list, latest_releases_by_channel};
use crate::workspace::Workspace;

mod dashboard;
mod health;
mod lines;

pub(in crate::tui_shell) use self::dashboard::dashboard_data;
pub(in crate::tui_shell) use self::lines::remote_status_lines;

#[derive(Debug, Clone)]
pub(in crate::tui_shell) struct DashboardData {
    pub(in crate::tui_shell) workflow_profile: WorkflowProfile,
    pub(in crate::tui_shell) healthz: Option<String>,
    pub(in crate::tui_shell) gates_total: usize,

    pub(in crate::tui_shell) inbox_total: usize,
    pub(in crate::tui_shell) inbox_pending: usize,
    pub(in crate::tui_shell) inbox_resolved: usize,
    pub(in crate::tui_shell) inbox_missing_local: usize,
    pub(in crate::tui_shell) latest_publication: Option<(String, String)>,

    pub(in crate::tui_shell) bundles_total: usize,
    pub(in crate::tui_shell) bundles_promotable: usize,
    pub(in crate::tui_shell) bundles_blocked: usize,
    pub(in crate::tui_shell) blocked_superpositions: usize,
    pub(in crate::tui_shell) blocked_approvals: usize,
    pub(in crate::tui_shell) pinned_bundles: usize,

    pub(in crate::tui_shell) promotion_state: Vec<(String, String)>,

    pub(in crate::tui_shell) releases_total: usize,
    pub(in crate::tui_shell) releases_channels: usize,
    pub(in crate::tui_shell) latest_releases: Vec<(String, String, String)>,

    pub(in crate::tui_shell) next_actions: Vec<String>,
}

fn new_dashboard_data() -> DashboardData {
    DashboardData {
        workflow_profile: WorkflowProfile::default(),
        healthz: None,
        gates_total: 0,

        inbox_total: 0,
        inbox_pending: 0,
        inbox_resolved: 0,
        inbox_missing_local: 0,
        latest_publication: None,

        bundles_total: 0,
        bundles_promotable: 0,
        bundles_blocked: 0,
        blocked_superpositions: 0,
        blocked_approvals: 0,
        pinned_bundles: 0,

        promotion_state: Vec::new(),

        releases_total: 0,
        releases_channels: 0,
        latest_releases: Vec::new(),

        next_actions: Vec::new(),
    }
}
