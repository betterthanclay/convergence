use std::any::Any;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use super::super::{RenderCtx, UiMode, View, render_view_chrome};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tui_shell) enum SettingsItemKind {
    ToggleTimestamps,
    ChunkingShow,
    ChunkingSet,
    ChunkingReset,
    RetentionShow,
    RetentionKeepLast,
    RetentionKeepDays,
    ToggleRetentionPruneSnaps,
    RetentionReset,
}

#[derive(Clone, Copy, Debug)]
pub(in crate::tui_shell) struct SettingsSnapshot {
    pub(in crate::tui_shell) chunk_size_mib: u64,
    pub(in crate::tui_shell) threshold_mib: u64,

    pub(in crate::tui_shell) retention_keep_last: Option<u64>,
    pub(in crate::tui_shell) retention_keep_days: Option<u64>,
    pub(in crate::tui_shell) retention_prune_snaps: bool,
    pub(in crate::tui_shell) retention_pinned: usize,
}

#[derive(Debug)]
pub(in crate::tui_shell) struct SettingsView {
    pub(in crate::tui_shell) updated_at: String,
    pub(in crate::tui_shell) items: Vec<SettingsItemKind>,
    pub(in crate::tui_shell) selected: usize,

    pub(in crate::tui_shell) snapshot: Option<SettingsSnapshot>,
}

impl SettingsView {
    pub(in crate::tui_shell) fn selected_kind(&self) -> Option<SettingsItemKind> {
        if self.items.is_empty() {
            return None;
        }
        Some(self.items[self.selected.min(self.items.len().saturating_sub(1))])
    }
}

impl View for SettingsView {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn mode(&self) -> UiMode {
        UiMode::Settings
    }

    fn title(&self) -> &str {
        "Settings"
    }

    fn updated_at(&self) -> &str {
        &self.updated_at
    }

    fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    fn move_down(&mut self) {
        if self.items.is_empty() {
            self.selected = 0;
            return;
        }
        let max = self.items.len().saturating_sub(1);
        self.selected = (self.selected + 1).min(max);
    }

    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect, ctx: &RenderCtx) {
        let inner = render_view_chrome(frame, self.title(), self.updated_at(), area);
        let parts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(inner);

        let mut state = ListState::default();
        if !self.items.is_empty() {
            state.select(Some(self.selected.min(self.items.len().saturating_sub(1))));
        }

        let mut rows = Vec::new();
        for k in &self.items {
            let row = match k {
                SettingsItemKind::ToggleTimestamps => {
                    format!("timestamps: {}", ctx.ts_mode.label())
                }
                SettingsItemKind::ChunkingShow => {
                    if let Some(s) = self.snapshot {
                        format!(
                            "chunking: show ({} / {} MiB)",
                            s.chunk_size_mib, s.threshold_mib
                        )
                    } else {
                        "chunking: show".to_string()
                    }
                }
                SettingsItemKind::ChunkingSet => {
                    if let Some(s) = self.snapshot {
                        format!(
                            "chunking: set... ({} / {} MiB)",
                            s.chunk_size_mib, s.threshold_mib
                        )
                    } else {
                        "chunking: set...".to_string()
                    }
                }
                SettingsItemKind::ChunkingReset => "chunking: reset".to_string(),
                SettingsItemKind::RetentionShow => "retention: show".to_string(),
                SettingsItemKind::RetentionKeepLast => {
                    if let Some(s) = self.snapshot {
                        format!(
                            "retention: keep_last... ({})",
                            s.retention_keep_last
                                .map(|n| n.to_string())
                                .unwrap_or_else(|| "unset".to_string())
                        )
                    } else {
                        "retention: keep_last...".to_string()
                    }
                }
                SettingsItemKind::RetentionKeepDays => {
                    if let Some(s) = self.snapshot {
                        format!(
                            "retention: keep_days... ({})",
                            s.retention_keep_days
                                .map(|n| n.to_string())
                                .unwrap_or_else(|| "unset".to_string())
                        )
                    } else {
                        "retention: keep_days...".to_string()
                    }
                }
                SettingsItemKind::ToggleRetentionPruneSnaps => {
                    if let Some(s) = self.snapshot {
                        format!(
                            "retention: prune_snaps (toggle) ({})",
                            if s.retention_prune_snaps { "on" } else { "off" }
                        )
                    } else {
                        "retention: prune_snaps (toggle)".to_string()
                    }
                }
                SettingsItemKind::RetentionReset => "retention: reset".to_string(),
            };
            rows.push(ListItem::new(row));
        }
        if rows.is_empty() {
            rows.push(ListItem::new("(empty)"));
        }

        let list = List::new(rows)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .title("(Enter: do it; /: commands)"),
            )
            .highlight_style(Style::default().bg(Color::DarkGray));
        frame.render_stateful_widget(list, parts[0], &mut state);

        let details = match self.selected_kind() {
            None => vec![Line::from("(no selection)")],
            Some(kind) => {
                let mut out = Vec::new();
                match kind {
                    SettingsItemKind::ToggleTimestamps => {
                        out.push(Line::from("Toggle timestamp display"));
                        out.push(Line::from(format!("current: {}", ctx.ts_mode.label())));
                    }
                    SettingsItemKind::ChunkingShow => {
                        out.push(Line::from("Show chunking settings"));
                        if let Some(s) = self.snapshot {
                            out.push(Line::from(format!(
                                "current: chunk_size={} MiB threshold={} MiB",
                                s.chunk_size_mib, s.threshold_mib
                            )));
                        }
                    }
                    SettingsItemKind::ChunkingSet => {
                        out.push(Line::from("Set chunking settings"));
                        if let Some(s) = self.snapshot {
                            out.push(Line::from(format!(
                                "current: {} {}",
                                s.chunk_size_mib, s.threshold_mib
                            )));
                        }
                        out.push(Line::from(
                            "Enter: edit (format: <chunk_size_mib> <threshold_mib>)",
                        ));
                    }
                    SettingsItemKind::ChunkingReset => {
                        out.push(Line::from("Reset chunking settings"));
                        out.push(Line::from("Enter: confirm + reset"));
                    }
                    SettingsItemKind::RetentionShow => {
                        out.push(Line::from("Show retention settings"));
                        if let Some(s) = self.snapshot {
                            out.push(Line::from(format!(
                                "current: keep_last={} keep_days={} prune_snaps={} pinned={}",
                                s.retention_keep_last
                                    .map(|n| n.to_string())
                                    .unwrap_or_else(|| "unset".to_string()),
                                s.retention_keep_days
                                    .map(|n| n.to_string())
                                    .unwrap_or_else(|| "unset".to_string()),
                                s.retention_prune_snaps,
                                s.retention_pinned
                            )));
                        }
                    }
                    SettingsItemKind::RetentionKeepLast => {
                        out.push(Line::from("Set retention keep_last"));
                        out.push(Line::from("Enter: edit (number of snaps, or 'unset')"));
                    }
                    SettingsItemKind::RetentionKeepDays => {
                        out.push(Line::from("Set retention keep_days"));
                        out.push(Line::from("Enter: edit (number of days, or 'unset')"));
                    }
                    SettingsItemKind::ToggleRetentionPruneSnaps => {
                        out.push(Line::from("Toggle retention prune_snaps"));
                    }
                    SettingsItemKind::RetentionReset => {
                        out.push(Line::from("Reset retention settings"));
                        out.push(Line::from("Enter: confirm + reset"));
                    }
                }
                out
            }
        };

        frame.render_widget(Paragraph::new(details).wrap(Wrap { trim: false }), parts[1]);
    }
}
