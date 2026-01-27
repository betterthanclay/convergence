use std::any::Any;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use super::super::{RenderCtx, UiMode, View, fmt_ts_ui, render_view_chrome};

#[derive(Debug)]
pub(in crate::tui_shell) struct GateGraphView {
    pub(in crate::tui_shell) updated_at: String,
    pub(in crate::tui_shell) graph: crate::remote::GateGraph,
    pub(in crate::tui_shell) selected: usize,
}

impl GateGraphView {
    pub(in crate::tui_shell) fn new(mut graph: crate::remote::GateGraph) -> Self {
        graph.gates.sort_by(|a, b| a.id.cmp(&b.id));
        Self {
            updated_at: super::super::app::now_ts(),
            graph,
            selected: 0,
        }
    }
}

impl View for GateGraphView {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn mode(&self) -> UiMode {
        UiMode::GateGraph
    }

    fn title(&self) -> &str {
        "Gate Graph"
    }

    fn updated_at(&self) -> &str {
        &self.updated_at
    }

    fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    fn move_down(&mut self) {
        if self.graph.gates.is_empty() {
            self.selected = 0;
            return;
        }
        let max = self.graph.gates.len().saturating_sub(1);
        self.selected = (self.selected + 1).min(max);
    }

    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect, _ctx: &RenderCtx) {
        let inner = render_view_chrome(frame, self.title(), self.updated_at(), area);
        let parts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(inner);

        let mut state = ListState::default();
        if !self.graph.gates.is_empty() {
            state.select(Some(
                self.selected.min(self.graph.gates.len().saturating_sub(1)),
            ));
        }

        let mut rows = Vec::new();
        for g in &self.graph.gates {
            let tag = if g.allow_releases { "" } else { "no-releases" };
            if tag.is_empty() {
                rows.push(ListItem::new(g.id.to_string()));
            } else {
                rows.push(ListItem::new(format!("{} {}", g.id, tag)));
            }
        }
        if rows.is_empty() {
            rows.push(ListItem::new("(empty)"));
        }

        let releases_enabled = self.graph.gates.iter().filter(|g| g.allow_releases).count();

        let list = List::new(rows)
            .block(Block::default().borders(Borders::BOTTOM).title(format!(
                "gates={} releases_enabled={} (/ for commands)",
                self.graph.gates.len(),
                releases_enabled
            )))
            .highlight_style(Style::default().bg(Color::DarkGray));
        frame.render_stateful_widget(list, parts[0], &mut state);

        let details = if self.graph.gates.is_empty() {
            vec![Line::from("(no selection)")]
        } else {
            let idx = self.selected.min(self.graph.gates.len().saturating_sub(1));
            let g = &self.graph.gates[idx];
            let mut out = Vec::new();
            out.push(Line::from(format!("id: {}", g.id)));
            out.push(Line::from(format!("name: {}", g.name)));
            if g.upstream.is_empty() {
                out.push(Line::from("upstream: (none)"));
            } else {
                out.push(Line::from(format!("upstream: {}", g.upstream.join(", "))));
            }
            out.push(Line::from(""));
            out.push(Line::from("policy:"));
            out.push(Line::from(format!("allow_releases: {}", g.allow_releases)));
            out.push(Line::from(format!(
                "allow_superpositions: {}",
                g.allow_superpositions
            )));
            out.push(Line::from(format!(
                "allow_metadata_only_publications: {}",
                g.allow_metadata_only_publications
            )));
            out.push(Line::from(format!(
                "required_approvals: {}",
                g.required_approvals
            )));
            out
        };
        frame.render_widget(
            Paragraph::new(details).wrap(Wrap { trim: false }).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("updated {}", fmt_ts_ui(self.updated_at()))),
            ),
            parts[1],
        );
    }
}
