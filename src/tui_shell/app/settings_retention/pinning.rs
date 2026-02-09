use super::*;

pub(super) fn pin_or_unpin_retention(app: &mut App, ws: &Workspace, sub: &str, args: &[String]) {
    if args.len() != 2 {
        app.push_error(format!("usage: retention {} <snap_id_prefix>", sub));
        return;
    }
    let prefix = &args[1];
    let snaps = match ws.list_snaps() {
        Ok(s) => s,
        Err(err) => {
            app.push_error(format!("list snaps: {:#}", err));
            return;
        }
    };
    let matches = snaps
        .iter()
        .filter(|s| s.id.starts_with(prefix))
        .map(|s| s.id.clone())
        .collect::<Vec<_>>();
    if matches.is_empty() {
        app.push_error(format!("no snap matches {}", prefix));
        return;
    }
    if matches.len() > 1 {
        app.push_error(format!("ambiguous snap prefix {}", prefix));
        return;
    }
    let snap_id = matches[0].clone();

    let mut cfg = match ws.store.read_config() {
        Ok(c) => c,
        Err(err) => {
            app.push_error(format!("read config: {:#}", err));
            return;
        }
    };
    let mut r = cfg.retention.unwrap_or_default();
    if sub == "pin" {
        if !r.pinned.iter().any(|p| p == &snap_id) {
            r.pinned.push(snap_id.clone());
        }
    } else {
        r.pinned.retain(|p| p != &snap_id);
    }
    cfg.retention = Some(r);
    if let Err(err) = ws.store.write_config(&cfg) {
        app.push_error(format!("write config: {:#}", err));
        return;
    }
    app.refresh_root_view();
    app.push_output(vec![format!("{} {}", sub, snap_id)]);
}
