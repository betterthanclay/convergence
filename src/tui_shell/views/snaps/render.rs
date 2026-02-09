use std::any::Any;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListState, Paragraph, Wrap};

use super::super::super::{RenderCtx, UiMode, View, render_view_chrome};
use super::SnapsView;
use super::details::details_lines;
use super::rows::list_rows;

impl View for SnapsView {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn mode(&self) -> UiMode {
        UiMode::Snaps
    }

    fn title(&self) -> &str {
        "History"
    }

    fn updated_at(&self) -> &str {
        &self.updated_at
    }

    fn move_up(&mut self) {
        self.selected_row = self.selected_row.saturating_sub(1);
    }

    fn move_down(&mut self) {
        let n = self.rows_len();
        if n == 0 {
            self.selected_row = 0;
            return;
        }
        let max = n.saturating_sub(1);
        self.selected_row = (self.selected_row + 1).min(max);
    }

    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect, ctx: &RenderCtx) {
        let inner = render_view_chrome(frame, self.title(), self.updated_at(), area);
        let parts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(inner);

        let mut state = ListState::default();
        let n_rows = self.rows_len();
        if n_rows > 0 {
            state.select(Some(self.selected_row.min(n_rows - 1)));
        }

        let list = List::new(list_rows(self, ctx))
            .block(Block::default().borders(Borders::BOTTOM).title(format!(
                "snaps{} (/: commands)",
                self.filter
                    .as_ref()
                    .map(|f| format!(" filter={}", f))
                    .unwrap_or_default()
            )))
            .highlight_style(Style::default().bg(Color::DarkGray));
        frame.render_stateful_widget(list, parts[0], &mut state);

        frame.render_widget(
            Paragraph::new(details_lines(self, ctx)).wrap(Wrap { trim: false }),
            parts[1],
        );
    }
}

pub(super) fn head_style() -> Style {
    Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD)
}
