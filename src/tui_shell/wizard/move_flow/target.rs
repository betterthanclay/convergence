use std::path::Path;

use super::super::super::App;
use super::prompts;

pub(super) fn move_wizard_to(app: &mut App, value: String) {
    let Some(ws) = app.require_workspace() else {
        return;
    };
    let Some(w) = app.move_wizard.as_mut() else {
        app.push_error("move wizard not active".to_string());
        return;
    };

    let Some(from) = w.from.clone() else {
        app.push_error("missing from".to_string());
        app.move_wizard = None;
        return;
    };

    let to = value.trim().trim_start_matches("./").to_string();
    if to.is_empty() {
        app.push_error("missing destination".to_string());
        return;
    }
    if to == from {
        app.push_error("destination must differ from source".to_string());
        return;
    }

    match ws.move_path(Path::new(&from), Path::new(&to)) {
        Ok(()) => {
            app.move_wizard = None;
            app.push_output(vec![format!("moved {} -> {}", from, to)]);
            app.refresh_root_view();
        }
        Err(err) => {
            prompts::open_to_retry_prompt(app, from, to, format!("{:#}", err));
        }
    }
}
