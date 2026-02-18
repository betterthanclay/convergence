use super::*;

pub(super) fn submit_wizard_text_input(app: &mut App, action: TextInputAction, value: String) {
    match action {
        TextInputAction::LoginUrl
        | TextInputAction::LoginToken
        | TextInputAction::LoginRepo
        | TextInputAction::LoginScope
        | TextInputAction::LoginGate => {
            app.continue_login_wizard(action, value);
        }

        TextInputAction::FetchKind
        | TextInputAction::FetchId
        | TextInputAction::FetchUser
        | TextInputAction::FetchOptions => {
            app.continue_fetch_wizard(action, value);
        }

        TextInputAction::PublishSnap
        | TextInputAction::PublishStart
        | TextInputAction::PublishScope
        | TextInputAction::PublishGate
        | TextInputAction::PublishMeta => {
            app.continue_publish_wizard(action, value);
        }

        TextInputAction::SyncStart
        | TextInputAction::SyncLane
        | TextInputAction::SyncClient
        | TextInputAction::SyncSnap => {
            app.continue_sync_wizard(action, value);
        }

        TextInputAction::ReleaseChannel | TextInputAction::ReleaseNotes => {
            app.continue_release_wizard(action, value);
        }

        TextInputAction::MemberAction
        | TextInputAction::MemberHandle
        | TextInputAction::MemberRole => {
            app.continue_member_wizard(action, value);
        }

        TextInputAction::LaneMemberAction
        | TextInputAction::LaneMemberLane
        | TextInputAction::LaneMemberHandle => {
            app.continue_lane_member_wizard(action, value);
        }

        TextInputAction::BrowseQuery => {
            app.continue_browse_wizard(action, value);
        }

        TextInputAction::MoveFrom | TextInputAction::MoveTo => {
            app.continue_move_wizard(action, value);
        }

        TextInputAction::BootstrapUrl
        | TextInputAction::BootstrapToken
        | TextInputAction::BootstrapHandle
        | TextInputAction::BootstrapDisplayName
        | TextInputAction::BootstrapRepo
        | TextInputAction::BootstrapScope
        | TextInputAction::BootstrapGate => {
            app.continue_bootstrap_wizard(action, value);
        }

        TextInputAction::GateGraphAddGateId
        | TextInputAction::GateGraphAddGateName
        | TextInputAction::GateGraphAddGateUpstream
        | TextInputAction::GateGraphEditUpstream
        | TextInputAction::GateGraphSetApprovals => {
            app.submit_gate_graph_text_input(action, value);
        }

        _ => app.push_error("unexpected text input action".to_string()),
    }
}
