use std::any::Any;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use super::super::{RenderCtx, UiMode, View, render_view_chrome};

mod render;

#[derive(Debug)]
pub(in crate::tui_shell) struct ReleasesView {
    pub(in crate::tui_shell) updated_at: String,
    pub(in crate::tui_shell) items: Vec<crate::remote::Release>,
    pub(in crate::tui_shell) selected: usize,
}

impl View for ReleasesView {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn mode(&self) -> UiMode {
        UiMode::Releases
    }

    fn title(&self) -> &str {
        "Releases"
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

        let mut rows: Vec<ListItem> = render::release_rows(&self.items, ctx)
            .into_iter()
            .map(ListItem::new)
            .collect();
        if rows.is_empty() {
            rows.push(ListItem::new("(empty)"));
        }

        let list = List::new(rows)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .title("channels (Enter: fetch; /: commands)"),
            )
            .highlight_style(Style::default().bg(Color::DarkGray));
        frame.render_stateful_widget(list, parts[0], &mut state);

        let details: Vec<Line> = render::release_details(&self.items, self.selected)
            .into_iter()
            .map(Line::from)
            .collect();
        frame.render_widget(Paragraph::new(details).wrap(Wrap { trim: false }), parts[1]);
    }
}
