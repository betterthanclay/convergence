use super::*;

pub(super) fn set_retention(app: &mut App, ws: &Workspace, args: &[String]) {
    let mut keep_last: Option<u64> = None;
    let mut keep_days: Option<u64> = None;
    let mut prune_snaps: Option<bool> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--keep-last" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    app.push_error("missing value for --keep-last".to_string());
                    return;
                };
                keep_last = v.parse::<u64>().ok();
            }
            "--keep-days" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    app.push_error("missing value for --keep-days".to_string());
                    return;
                };
                keep_days = v.parse::<u64>().ok();
            }
            "--prune-snaps" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    app.push_error("missing value for --prune-snaps".to_string());
                    return;
                };
                prune_snaps = match v.as_str() {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None,
                };
            }
            _ => {
                app.push_error(
                    "usage: settings retention set [--keep-last N] [--keep-days N] [--prune-snaps true|false]"
                        .to_string(),
                );
                return;
            }
        }
        i += 1;
    }

    let mut cfg = match ws.store.read_config() {
        Ok(c) => c,
        Err(err) => {
            app.push_error(format!("read config: {:#}", err));
            return;
        }
    };
    let mut r = cfg.retention.unwrap_or_default();
    if keep_last.is_some() {
        r.keep_last = keep_last;
    }
    if keep_days.is_some() {
        r.keep_days = keep_days;
    }
    if let Some(v) = prune_snaps {
        r.prune_snaps = v;
    }
    cfg.retention = Some(r);
    if let Err(err) = ws.store.write_config(&cfg) {
        app.push_error(format!("write config: {:#}", err));
        return;
    }
    app.refresh_root_view();
    app.push_output(vec!["updated retention config".to_string()]);
}

pub(super) fn reset_retention(app: &mut App, ws: &Workspace) {
    let mut cfg = match ws.store.read_config() {
        Ok(c) => c,
        Err(err) => {
            app.push_error(format!("read config: {:#}", err));
            return;
        }
    };
    cfg.retention = None;
    if let Err(err) = ws.store.write_config(&cfg) {
        app.push_error(format!("write config: {:#}", err));
        return;
    }
    app.refresh_root_view();
    app.push_output(vec!["reset retention config".to_string()]);
}
