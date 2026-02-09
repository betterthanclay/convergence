use crate::tui_shell::App;

use super::MemberAction;

pub(super) fn finish_member_wizard(app: &mut App) {
    let Some(w) = app.member_wizard.clone() else {
        app.push_error("member wizard not active".to_string());
        return;
    };
    app.member_wizard = None;

    let client = match app.remote_client() {
        Some(c) => c,
        None => return,
    };
    let Some(action) = w.action else {
        app.push_error("member: missing action".to_string());
        return;
    };
    let Some(handle) = w.handle else {
        app.push_error("member: missing handle".to_string());
        return;
    };

    match action {
        MemberAction::Add => match client.add_repo_member(&handle, &w.role) {
            Ok(()) => {
                app.push_output(vec![format!("added {} ({})", handle, w.role)]);
                app.refresh_root_view();
            }
            Err(err) => app.push_error(format!("member add: {:#}", err)),
        },
        MemberAction::Remove => match client.remove_repo_member(&handle) {
            Ok(()) => {
                app.push_output(vec![format!("removed {}", handle)]);
                app.refresh_root_view();
            }
            Err(err) => app.push_error(format!("member remove: {:#}", err)),
        },
    }
}
