use super::*;

pub(super) fn is_settings_action(action: &TextInputAction) -> bool {
    matches!(
        action,
        TextInputAction::ChunkingSet
            | TextInputAction::RetentionKeepLast
            | TextInputAction::RetentionKeepDays
    )
}

pub(super) fn apply_settings_text_input(app: &mut App, action: TextInputAction, value: String) {
    let Some(ws) = app.require_workspace() else {
        return;
    };

    match action {
        TextInputAction::ChunkingSet => apply_chunking_set(app, &ws, value),
        TextInputAction::RetentionKeepLast | TextInputAction::RetentionKeepDays => {
            apply_retention_update(app, &ws, action, value);
        }
        _ => app.push_error("unexpected settings text input action".to_string()),
    }
}

fn apply_chunking_set(app: &mut App, ws: &Workspace, value: String) {
    let norm = value.replace(',', " ");
    let parts = norm.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 2 {
        app.push_error("format: <chunk_size_mib> <threshold_mib>".to_string());
        return;
    }
    let chunk_size_mib = match parts[0].parse::<u64>() {
        Ok(n) if n > 0 => n,
        _ => {
            app.push_error("invalid chunk_size_mib".to_string());
            return;
        }
    };
    let threshold_mib = match parts[1].parse::<u64>() {
        Ok(n) if n > 0 => n,
        _ => {
            app.push_error("invalid threshold_mib".to_string());
            return;
        }
    };
    if threshold_mib < chunk_size_mib {
        app.push_error("threshold must be >= chunk_size".to_string());
        return;
    }

    let mut cfg = match ws.store.read_config() {
        Ok(c) => c,
        Err(err) => {
            app.push_error(format!("read config: {:#}", err));
            return;
        }
    };
    cfg.chunking = Some(ChunkingConfig {
        chunk_size: chunk_size_mib * 1024 * 1024,
        threshold: threshold_mib * 1024 * 1024,
    });
    if let Err(err) = ws.store.write_config(&cfg) {
        app.push_error(format!("write config: {:#}", err));
        return;
    }

    app.refresh_root_view();
    app.refresh_settings_view();
    app.push_output(vec!["updated chunking config".to_string()]);
}

fn apply_retention_update(app: &mut App, ws: &Workspace, action: TextInputAction, value: String) {
    let v = value.trim();
    let v_lc = v.to_lowercase();
    let parsed = if v_lc == "unset" || v_lc == "none" {
        None
    } else {
        match v.parse::<u64>() {
            Ok(n) if n > 0 => Some(n),
            _ => {
                app.push_error("expected a positive number (or 'unset')".to_string());
                return;
            }
        }
    };

    let mut cfg = match ws.store.read_config() {
        Ok(c) => c,
        Err(err) => {
            app.push_error(format!("read config: {:#}", err));
            return;
        }
    };
    let mut retention = cfg.retention.unwrap_or_default();
    match action {
        TextInputAction::RetentionKeepLast => retention.keep_last = parsed,
        TextInputAction::RetentionKeepDays => retention.keep_days = parsed,
        _ => {}
    }
    cfg.retention = Some(retention);
    if let Err(err) = ws.store.write_config(&cfg) {
        app.push_error(format!("write config: {:#}", err));
        return;
    }

    app.refresh_root_view();
    app.refresh_settings_view();
    match action {
        TextInputAction::RetentionKeepLast => {
            app.push_output(vec!["updated retention keep_last".to_string()]);
        }
        TextInputAction::RetentionKeepDays => {
            app.push_output(vec!["updated retention keep_days".to_string()]);
        }
        _ => {}
    }
}
