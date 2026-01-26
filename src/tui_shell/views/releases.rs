use std::any::Any;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use super::super::{RenderCtx, UiMode, View, fmt_ts_list, fmt_ts_ui, render_view_chrome};

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

        let mut rows = Vec::new();
        for r in &self.items {
            let short = r.bundle_id.chars().take(8).collect::<String>();
            rows.push(ListItem::new(format!(
                "{} {} {}",
                r.channel,
                short,
                fmt_ts_list(&r.released_at, ctx)
            )));
        }
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

        let details = if self.items.is_empty() {
            vec![Line::from("(no selection)")]
        } else {
            let idx = self.selected.min(self.items.len().saturating_sub(1));
            let r = &self.items[idx];
            let mut out = Vec::new();
            out.push(Line::from(format!("channel: {}", r.channel)));
            out.push(Line::from(format!("bundle: {}", r.bundle_id)));
            out.push(Line::from(format!("scope: {}", r.scope)));
            out.push(Line::from(format!("gate: {}", r.gate)));
            out.push(Line::from(format!(
                "released_at: {}",
                fmt_ts_ui(&r.released_at)
            )));
            out.push(Line::from(format!("released_by: {}", r.released_by)));
            if let Some(n) = &r.notes {
                out.push(Line::from(""));
                out.push(Line::from(format!("notes: {}", n)));
            }
            out
        };
        frame.render_widget(Paragraph::new(details).wrap(Wrap { trim: false }), parts[1]);
    }
}
