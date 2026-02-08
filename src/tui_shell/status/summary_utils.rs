#[derive(Clone, Copy, Debug, Default)]
pub(in crate::tui_shell) struct ChangeSummary {
    pub(in crate::tui_shell) added: usize,
    pub(in crate::tui_shell) modified: usize,
    pub(in crate::tui_shell) deleted: usize,
    pub(in crate::tui_shell) renamed: usize,
}

impl ChangeSummary {
    pub(in crate::tui_shell) fn total(&self) -> usize {
        self.added + self.modified + self.deleted + self.renamed
    }
}

pub(in crate::tui_shell) fn extract_change_summary(
    mut lines: Vec<String>,
) -> (ChangeSummary, Vec<String>) {
    let mut sum = ChangeSummary::default();

    // Local status_lines emits either:
    // - "changes: X added, Y modified, Z deleted"
    // - "changes: X added, Y modified, Z deleted, R renamed"
    for i in 0..lines.len() {
        let line = lines[i].trim();
        if !line.starts_with("changes:") {
            continue;
        }

        let rest = line.trim_start_matches("changes:").trim();
        let parts: Vec<&str> = rest.split(',').map(|p| p.trim()).collect();
        for p in parts {
            let mut it = p.split_whitespace();
            let Some(n) = it.next() else {
                continue;
            };
            let Ok(n) = n.parse::<usize>() else {
                continue;
            };
            let Some(kind) = it.next() else {
                continue;
            };
            match kind {
                "added" => sum.added = n,
                "modified" => sum.modified = n,
                "deleted" => sum.deleted = n,
                "renamed" => sum.renamed = n,
                _ => {}
            }
        }

        lines.remove(i);
        break;
    }

    (sum, lines)
}

pub(in crate::tui_shell) fn extract_baseline_compact(lines: &[String]) -> Option<String> {
    for l in lines {
        let l = l.trim();
        if let Some(rest) = l.strip_prefix("baseline:") {
            let rest = rest.trim();
            if rest.starts_with('(') {
                return None;
            }
            // Expected: "<short> <time>".
            return Some(rest.to_string());
        }
    }
    None
}

pub(in crate::tui_shell) fn extract_change_keys(lines: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for l in lines {
        let line = l.trim();
        let base = line.split_once(" (").map(|(a, _)| a).unwrap_or(line);

        if let Some(rest) = base.strip_prefix("A ") {
            out.push(format!("A {}", rest.trim()));
            continue;
        }
        if let Some(rest) = base.strip_prefix("M ") {
            out.push(format!("M {}", rest.trim()));
            continue;
        }
        if let Some(rest) = base.strip_prefix("D ") {
            out.push(format!("D {}", rest.trim()));
            continue;
        }
        if let Some(rest) = base.strip_prefix("R* ") {
            out.push(format!("R {}", rest.trim()));
            continue;
        }
        if let Some(rest) = base.strip_prefix("R ") {
            out.push(format!("R {}", rest.trim()));
            continue;
        }
    }
    out
}

pub(in crate::tui_shell) fn jaccard_similarity(a: &[String], b: &[String]) -> f64 {
    use std::collections::HashSet;
    let sa: HashSet<&str> = a.iter().map(|s| s.as_str()).collect();
    let sb: HashSet<&str> = b.iter().map(|s| s.as_str()).collect();
    if sa.is_empty() && sb.is_empty() {
        return 1.0;
    }
    let inter = sa.intersection(&sb).count();
    let union = sa.union(&sb).count();
    if union == 0 {
        1.0
    } else {
        inter as f64 / union as f64
    }
}

pub(in crate::tui_shell) fn collapse_blank_lines(lines: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    let mut prev_blank = false;
    for l in lines {
        let blank = l.trim().is_empty();
        if blank && prev_blank {
            continue;
        }
        prev_blank = blank;
        out.push(l);
    }
    out
}
