use std::any::Any;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use super::super::status::ChangeSummary;
use super::super::{RenderCtx, UiMode, View, fmt_ts_list, fmt_ts_ui, render_view_chrome};

#[derive(Debug)]
pub(in crate::tui_shell) struct SnapsView {
    pub(in crate::tui_shell) updated_at: String,
    pub(in crate::tui_shell) filter: Option<String>,
    pub(in crate::tui_shell) all_items: Vec<crate::model::SnapRecord>,
    pub(in crate::tui_shell) items: Vec<crate::model::SnapRecord>,
    pub(in crate::tui_shell) selected_row: usize,

    pub(in crate::tui_shell) head_id: Option<String>,

    pub(in crate::tui_shell) pending_changes: Option<ChangeSummary>,
}

impl SnapsView {
    fn has_pending_row(&self) -> bool {
        self.pending_changes.is_some_and(|s| s.total() > 0)
    }

    fn rows_len(&self) -> usize {
        let mut n = self.items.len();
        if self.has_pending_row() {
            n += 1;
        }
        if self.items.is_empty() {
            n += 1;
        }
        n
    }

    pub(in crate::tui_shell) fn selected_is_pending(&self) -> bool {
        self.has_pending_row() && self.selected_row.min(self.rows_len().saturating_sub(1)) == 0
    }

    pub(in crate::tui_shell) fn selected_snap_index(&self) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }
        let row = self.selected_row.min(self.rows_len().saturating_sub(1));
        let idx = if self.has_pending_row() {
            if row == 0 {
                return None;
            }
            row - 1
        } else {
            row
        };
        if idx < self.items.len() {
            Some(idx)
        } else {
            None
        }
    }
}

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

        let mut rows = Vec::new();

        let has_pending = self.has_pending_row();
        let head_style = Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD);

        if let Some(sum) = self.pending_changes
            && has_pending
        {
            let total = sum.total();
            let label = if total == 1 { "change" } else { "changes" };
            rows.push(ListItem::new(format!("> {} {}", total, label)).style(head_style));
        }

        for s in &self.items {
            let is_head = self.head_id.as_deref() == Some(s.id.as_str());
            let sid = s.id.chars().take(8).collect::<String>();
            let msg = s.message.clone().unwrap_or_default();
            let marker = if is_head { "*" } else { " " };
            let id_style = if is_head && !has_pending {
                head_style
            } else {
                Style::default()
            };
            let row = if msg.is_empty() {
                format!("{} {} {}", marker, sid, fmt_ts_list(&s.created_at, ctx))
            } else {
                format!(
                    "{} {} {} {}",
                    marker,
                    sid,
                    fmt_ts_list(&s.created_at, ctx),
                    msg
                )
            };

            rows.push(ListItem::new(row).style(id_style));
        }

        if self.items.is_empty() {
            rows.push(ListItem::new("(no snaps)"));
        }

        let list = List::new(rows)
            .block(Block::default().borders(Borders::BOTTOM).title(format!(
                "snaps{} (/: commands)",
                self.filter
                    .as_ref()
                    .map(|f| format!(" filter={}", f))
                    .unwrap_or_default()
            )))
            .highlight_style(Style::default().bg(Color::DarkGray));
        frame.render_stateful_widget(list, parts[0], &mut state);

        let details = if has_pending && self.selected_is_pending() {
            let sum = self.pending_changes.unwrap_or_default();
            let total = sum.total();
            let label = if total == 1 { "change" } else { "changes" };
            vec![
                Line::from(format!("pending: {} {}", total, label)),
                Line::from(format!(
                    "A:{} M:{} D:{} R:{}",
                    sum.added, sum.modified, sum.deleted, sum.renamed
                )),
                Line::from(""),
                Line::from("Enter: snap (or rotate hint to revert)"),
            ]
        } else if let Some(idx) = self.selected_snap_index() {
            let s = &self.items[idx];
            let mut out = Vec::new();
            if self.head_id.as_deref() == Some(s.id.as_str()) {
                out.push(Line::from("active: yes"));
            }
            out.push(Line::from(format!("id: {}", s.id)));
            out.push(Line::from(format!(
                "created_at: {}",
                fmt_ts_ui(&s.created_at)
            )));
            if let Some(msg) = &s.message
                && !msg.is_empty()
            {
                out.push(Line::from(format!("message: {}", msg)));
            }
            out.push(Line::from(format!(
                "root_manifest: {}",
                s.root_manifest.as_str()
            )));
            out.push(Line::from(format!(
                "stats: files={} dirs={} symlinks={} bytes={}",
                s.stats.files, s.stats.dirs, s.stats.symlinks, s.stats.bytes
            )));
            out
        } else {
            vec![Line::from("(no selection)")]
        };
        frame.render_widget(Paragraph::new(details).wrap(Wrap { trim: false }), parts[1]);
    }
}
