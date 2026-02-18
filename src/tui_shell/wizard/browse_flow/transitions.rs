use super::*;
use crate::tui_shell::App;

fn parse_limit(raw: &str) -> Result<Option<usize>, String> {
    let v = raw.trim();
    if v.is_empty() || v.eq_ignore_ascii_case("none") {
        return Ok(None);
    }
    match v.parse::<usize>() {
        Ok(n) => Ok(Some(n)),
        Err(_) => Err("invalid limit (expected number or none)".to_string()),
    }
}

fn parse_filter(raw: &str) -> Option<String> {
    let v = raw.trim();
    if v.is_empty() || v.eq_ignore_ascii_case("none") {
        None
    } else {
        Some(v.to_string())
    }
}

fn apply_compact_query(w: &mut BrowseWizard, raw: &str) -> Result<(), String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(());
    }

    if raw.contains('=') {
        for part in raw.split_whitespace() {
            let Some((k, v)) = part.split_once('=') else {
                return Err(format!("invalid field: {}", part));
            };
            match k {
                "scope" => {
                    if !v.is_empty() {
                        w.scope = v.to_string();
                    }
                }
                "gate" => {
                    if !v.is_empty() {
                        w.gate = v.to_string();
                    }
                }
                "filter" => w.filter = parse_filter(v),
                "limit" => w.limit = parse_limit(v)?,
                _ => return Err(format!("unknown field: {}", k)),
            }
        }
        return Ok(());
    }

    let parts = raw.split_whitespace().collect::<Vec<_>>();
    if let Some(scope) = parts.first() {
        w.scope = (*scope).to_string();
    }
    if let Some(gate) = parts.get(1) {
        w.gate = (*gate).to_string();
    }
    if let Some(filter) = parts.get(2) {
        w.filter = parse_filter(filter);
    }
    if let Some(limit) = parts.get(3) {
        w.limit = parse_limit(limit)?;
    }
    Ok(())
}

pub(super) fn continue_browse_wizard(app: &mut App, action: TextInputAction, value: String) {
    if app.browse_wizard.is_none() {
        app.push_error("browse wizard not active".to_string());
        return;
    }

    match action {
        TextInputAction::BrowseQuery => {
            if let Some(w) = app.browse_wizard.as_mut()
                && let Err(err) = apply_compact_query(w, &value)
            {
                app.push_error(format!("browse edit: {}", err));
                return;
            }
            app.finish_browse_wizard();
        }
        _ => {
            app.push_error("unexpected browse wizard input".to_string());
        }
    }
}
