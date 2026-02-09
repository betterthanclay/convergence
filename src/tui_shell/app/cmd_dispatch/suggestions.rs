use super::*;

impl App {
    pub(in crate::tui_shell::app) fn recompute_suggestions(&mut self) {
        let show = self.input.buf.trim_start().starts_with('/');
        let q = self.input.buf.trim_start_matches('/').trim().to_lowercase();
        if q.is_empty() {
            if show {
                let mut defs = self.available_command_defs();
                defs.sort_by(|a, b| a.name.cmp(b.name));
                self.suggestions = defs;
                self.suggestion_selected = 0;
            } else {
                self.suggestions.clear();
                self.suggestion_selected = 0;
            }
            return;
        }

        let first = q.split_whitespace().next().unwrap_or("");
        if first.is_empty() {
            self.suggestions.clear();
            self.suggestion_selected = 0;
            return;
        }

        let mut defs = self.available_command_defs();
        defs.sort_by(|a, b| a.name.cmp(b.name));

        let mut scored = Vec::new();
        for d in defs {
            let mut best = score_match(first, d.name);
            for &a in d.aliases {
                best = best.max(score_match(first, a));
            }
            if best > 0 {
                scored.push((best, d));
            }
        }

        let hint_order = self.primary_hint_commands();
        sort_scored_suggestions(&mut scored, &hint_order);
        self.suggestions = scored.into_iter().map(|(_, d)| d).collect();
        self.suggestion_selected = self.suggestion_selected.min(self.suggestions.len());
    }

    pub(in crate::tui_shell::app) fn apply_selected_suggestion(&mut self) {
        if self.suggestions.is_empty() {
            return;
        }
        let show = self.input.buf.trim_start().starts_with('/');
        let sel = self
            .suggestion_selected
            .min(self.suggestions.len().saturating_sub(1));
        let cmd = self.suggestions[sel].name;

        let prefix = if show { "/" } else { "" };
        let raw = self.input.buf.trim_start_matches('/');
        let trimmed = raw.trim_start();
        let mut iter = trimmed.splitn(2, char::is_whitespace);
        let first = iter.next().unwrap_or("");
        let rest = iter.next().unwrap_or("");

        if first.is_empty() || rest.is_empty() {
            self.input.set(format!("{}{} ", prefix, cmd));
        } else {
            self.input
                .set(format!("{}{} {}", prefix, cmd, rest.trim_start()));
        }
        self.recompute_suggestions();
    }
}
