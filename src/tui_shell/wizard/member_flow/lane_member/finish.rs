use crate::tui_shell::App;

use super::MemberAction;

pub(super) fn finish_lane_member_wizard(app: &mut App) {
    let Some(w) = app.lane_member_wizard.clone() else {
        app.push_error("lane-member wizard not active".to_string());
        return;
    };
    app.lane_member_wizard = None;

    let client = match app.remote_client() {
        Some(c) => c,
        None => return,
    };
    let Some(action) = w.action else {
        app.push_error("lane-member: missing action".to_string());
        return;
    };
    let Some(lane) = w.lane else {
        app.push_error("lane-member: missing lane".to_string());
        return;
    };
    let Some(handle) = w.handle else {
        app.push_error("lane-member: missing handle".to_string());
        return;
    };

    match action {
        MemberAction::Add => match client.add_lane_member(&lane, &handle) {
            Ok(()) => {
                app.push_output(vec![format!("added {} to lane {}", handle, lane)]);
                app.refresh_root_view();
            }
            Err(err) => app.push_error(format!("lane-member add: {:#}", err)),
        },
        MemberAction::Remove => match client.remove_lane_member(&lane, &handle) {
            Ok(()) => {
                app.push_output(vec![format!("removed {} from lane {}", handle, lane)]);
                app.refresh_root_view();
            }
            Err(err) => app.push_error(format!("lane-member remove: {:#}", err)),
        },
    }
}
