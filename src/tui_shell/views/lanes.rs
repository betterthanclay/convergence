use std::any::Any;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use super::super::{RenderCtx, UiMode, View, fmt_ts_list, fmt_ts_ui, render_view_chrome};

#[derive(Clone, Debug)]
pub(in crate::tui_shell) struct LaneHeadItem {
    pub(in crate::tui_shell) lane_id: String,
    pub(in crate::tui_shell) user: String,
    pub(in crate::tui_shell) head: Option<crate::remote::LaneHead>,
    pub(in crate::tui_shell) local: bool,
}

#[derive(Debug)]
pub(in crate::tui_shell) struct LanesView {
    pub(in crate::tui_shell) updated_at: String,
    pub(in crate::tui_shell) items: Vec<LaneHeadItem>,
    pub(in crate::tui_shell) selected: usize,
}

impl View for LanesView {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn mode(&self) -> UiMode {
        UiMode::Lanes
    }

    fn title(&self) -> &str {
        "Lanes"
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
        for it in &self.items {
            let head = it
                .head
                .as_ref()
                .map(|h| h.snap_id.chars().take(8).collect::<String>())
                .unwrap_or_else(|| "-".to_string());
            let ts = it
                .head
                .as_ref()
                .map(|h| fmt_ts_list(&h.updated_at, ctx))
                .unwrap_or_else(|| "".to_string());
            let local = if it.local { " local" } else { "" };
            if ts.is_empty() {
                rows.push(ListItem::new(format!(
                    "{:<10} {:<10} {}{}",
                    it.lane_id, it.user, head, local
                )));
            } else {
                rows.push(ListItem::new(format!(
                    "{:<10} {:<10} {} {}{}",
                    it.lane_id, it.user, head, ts, local
                )));
            }
        }
        if rows.is_empty() {
            rows.push(ListItem::new("(empty)"));
        }

        let list = List::new(rows)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .title("(Enter: fetch; /: commands)".to_string()),
            )
            .highlight_style(Style::default().bg(Color::DarkGray));
        frame.render_stateful_widget(list, parts[0], &mut state);

        let details = if self.items.is_empty() {
            vec![Line::from("(no selection)")]
        } else {
            let idx = self.selected.min(self.items.len().saturating_sub(1));
            let it = &self.items[idx];
            let mut out = Vec::new();
            out.push(Line::from(format!("lane: {}", it.lane_id)));
            out.push(Line::from(format!("user: {}", it.user)));
            if let Some(h) = &it.head {
                out.push(Line::from(format!("snap: {}", h.snap_id)));
                out.push(Line::from(format!(
                    "updated_at: {}",
                    fmt_ts_ui(&h.updated_at)
                )));
                if let Some(cid) = &h.client_id {
                    out.push(Line::from(format!("client_id: {}", cid)));
                }
            } else {
                out.push(Line::from("snap: (none)"));
            }
            out.push(Line::from(format!("local: {}", it.local)));
            out
        };
        frame.render_widget(Paragraph::new(details).wrap(Wrap { trim: false }), parts[1]);
    }
}
