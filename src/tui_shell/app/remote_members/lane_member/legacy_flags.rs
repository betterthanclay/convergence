use super::*;

pub(super) fn handle_legacy_flags(app: &mut App, args: &[String]) {
    // Back-compat: accept legacy flag form.
    let client = match app.remote_client() {
        Some(c) => c,
        None => {
            app.start_login_wizard();
            return;
        }
    };

    let sub = &args[0];
    let mut lane: Option<String> = None;
    let mut handle: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--lane" => {
                i += 1;
                if i >= args.len() {
                    app.push_error("missing value for --lane".to_string());
                    return;
                }
                lane = Some(args[i].clone());
            }
            "--handle" => {
                i += 1;
                if i >= args.len() {
                    app.push_error("missing value for --handle".to_string());
                    return;
                }
                handle = Some(args[i].clone());
            }
            a => {
                app.push_error(format!("unknown arg: {}", a));
                return;
            }
        }
        i += 1;
    }

    let Some(lane) = lane else {
        app.push_error("missing --lane".to_string());
        return;
    };
    let Some(handle) = handle else {
        app.push_error("missing --handle".to_string());
        return;
    };

    match sub.as_str() {
        "add" => match client.add_lane_member(&lane, &handle) {
            Ok(()) => {
                app.push_output(vec![format!("added {} to lane {}", handle, lane)]);
                app.refresh_root_view();
            }
            Err(err) => app.push_error(format!("lane-member add: {:#}", err)),
        },
        "remove" | "rm" => match client.remove_lane_member(&lane, &handle) {
            Ok(()) => {
                app.push_output(vec![format!("removed {} from lane {}", handle, lane)]);
                app.refresh_root_view();
            }
            Err(err) => app.push_error(format!("lane-member remove: {:#}", err)),
        },
        _ => app.start_lane_member_wizard(None),
    }
}
