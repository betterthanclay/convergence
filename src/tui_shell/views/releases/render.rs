use super::super::super::{RenderCtx, fmt_ts_list, fmt_ts_ui};

pub(super) fn release_rows(items: &[crate::remote::Release], ctx: &RenderCtx) -> Vec<String> {
    let mut rows = Vec::new();
    for release in items {
        let short = release.bundle_id.chars().take(8).collect::<String>();
        rows.push(format!(
            "{} {} {}",
            release.channel,
            short,
            fmt_ts_list(&release.released_at, ctx)
        ));
    }
    rows
}

pub(super) fn release_details(items: &[crate::remote::Release], selected: usize) -> Vec<String> {
    if items.is_empty() {
        return vec!["(no selection)".to_string()];
    }

    let idx = selected.min(items.len().saturating_sub(1));
    let release = &items[idx];
    let mut out = Vec::new();
    out.push(format!("channel: {}", release.channel));
    out.push(format!("bundle: {}", release.bundle_id));
    out.push(format!("scope: {}", release.scope));
    out.push(format!("gate: {}", release.gate));
    out.push(format!("released_at: {}", fmt_ts_ui(&release.released_at)));
    out.push(format!("released_by: {}", release.released_by));
    if let Some(notes) = &release.notes {
        out.push(String::new());
        out.push(format!("notes: {}", notes));
    }
    out
}
