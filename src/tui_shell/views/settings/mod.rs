use std::any::Any;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListState, Paragraph, Wrap};

use super::super::{RenderCtx, UiMode, View, render_view_chrome};

mod details;
mod list_rows;

use self::details::detail_lines;
use self::list_rows::list_rows;

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

        let list = List::new(list_rows(self, ctx))
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .title("(Enter: do it; /: commands)"),
            )
            .highlight_style(Style::default().bg(Color::DarkGray));
        frame.render_stateful_widget(list, parts[0], &mut state);

        frame.render_widget(
            Paragraph::new(detail_lines(self, ctx)).wrap(Wrap { trim: false }),
            parts[1],
        );
    }
}
