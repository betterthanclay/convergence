use super::super::super::*;

mod publish;
mod resolve;

pub(super) fn cmd_superpositions_apply_mode(app: &mut App, args: &[String]) {
    let mut publish_flag = false;
    for a in args {
        match a.as_str() {
            "--publish" | "publish" => publish_flag = true,
            _ => {
                app.push_error("usage: apply [publish]".to_string());
                return;
            }
        }
    }

    let Some(ws) = app.require_workspace() else {
        return;
    };

    let Some((bundle_id, root_manifest)) = app
        .current_view::<SuperpositionsView>()
        .map(|v| (v.bundle_id.clone(), v.root_manifest.clone()))
    else {
        app.push_error("not in superpositions mode".to_string());
        return;
    };

    let snap = match resolve::build_resolved_snap(app, &ws, &bundle_id, &root_manifest) {
        Ok(s) => s,
        Err(()) => return,
    };
    if let Err(err) = ws.store.put_snap(&snap) {
        app.push_error(format!("write snap: {:#}", err));
        return;
    }

    let pub_id = if publish_flag {
        match publish::publish_resolved_snap(app, &ws, &bundle_id, &root_manifest, &snap) {
            Ok(pid) => pid,
            Err(()) => return,
        }
    } else {
        None
    };

    if let Some(pid) = pub_id {
        app.push_output(vec![format!(
            "resolved snap {} (published {})",
            snap.id, pid
        )]);
    } else {
        app.push_output(vec![format!("resolved snap {}", snap.id)]);
    }
}
