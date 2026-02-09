use crate::tui_shell::App;

pub(super) fn finish_publish_wizard(app: &mut App) {
    let Some(w) = app.publish_wizard.clone() else {
        app.push_error("publish wizard not active".to_string());
        return;
    };
    app.publish_wizard = None;

    let mut argv: Vec<String> = Vec::new();
    if let Some(s) = w.snap {
        argv.extend(["--snap-id".to_string(), s]);
    }
    if let Some(s) = w.scope {
        argv.extend(["--scope".to_string(), s]);
    }
    if let Some(g) = w.gate {
        argv.extend(["--gate".to_string(), g]);
    }
    if w.meta {
        argv.push("--metadata-only".to_string());
    }

    app.cmd_publish_impl(&argv);
}
