use std::any::Any;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::workspace::Workspace;

use super::super::app::{now_ts, root_ctx_color};
use super::super::status::{
    ChangeSummary, DashboardData, collapse_blank_lines, dashboard_data, extract_baseline_compact,
    extract_change_keys, extract_change_summary, jaccard_similarity, local_status_lines,
};
use super::super::view::render_view_chrome_with_header;
use super::super::{RenderCtx, RootContext, UiMode, View, fmt_ts_ui};

#[derive(Debug)]
pub(in crate::tui_shell) struct RootView {
    pub(in crate::tui_shell) updated_at: String,
    pub(in crate::tui_shell) ctx: RootContext,
    scroll: usize,
    lines: Vec<String>,
    pub(in crate::tui_shell) remote_auth_block_lines: Option<Vec<String>>,
    pub(in crate::tui_shell) change_summary: ChangeSummary,
    baseline_compact: Option<String>,
    change_keys: Vec<String>,

    remote_dashboard: Option<DashboardData>,
    remote_err: Option<String>,
}

impl RootView {
    pub(in crate::tui_shell) fn new(ctx: RootContext) -> Self {
        Self {
            updated_at: now_ts(),
            ctx,
            scroll: 0,
            lines: Vec::new(),
            remote_auth_block_lines: None,
            change_summary: ChangeSummary::default(),
            baseline_compact: None,
            change_keys: Vec::new(),

            remote_dashboard: None,
            remote_err: None,
        }
    }

    pub(in crate::tui_shell) fn refresh(&mut self, ws: Option<&Workspace>, ctx: &RenderCtx) {
        let prev_lines_len = self.lines.len();
        let prev_baseline = self.baseline_compact.clone();
        let prev_keys = self.change_keys.clone();

        let lines = match (self.ctx, ws) {
            (_, None) => vec!["No workspace".to_string()],
            (RootContext::Local, Some(ws)) => {
                local_status_lines(ws, ctx).unwrap_or_else(|e| vec![format!("status: {:#}", e)])
            }
            (RootContext::Remote, Some(ws)) => {
                if let Some(lines) = self.remote_auth_block_lines.clone() {
                    lines
                } else {
                    match dashboard_data(ws, ctx) {
                        Ok(d) => {
                            self.remote_dashboard = Some(d);
                            self.remote_err = None;
                            Vec::new()
                        }
                        Err(err) => {
                            self.remote_dashboard = None;
                            let s = sanitize_dashboard_err(&format!("{:#}", err));
                            self.remote_err = Some(s.clone());
                            vec![s]
                        }
                    }
                }
            }
        };

        if self.ctx == RootContext::Local {
            let (summary, lines) = extract_change_summary(lines);
            self.change_summary = summary;
            self.baseline_compact = extract_baseline_compact(&lines);

            let new_lines = collapse_blank_lines(lines);
            let new_keys = extract_change_keys(&new_lines);
            self.change_keys = new_keys.clone();

            // Preserve scroll position unless the change list shifts substantially.
            let significant = {
                if prev_baseline != self.baseline_compact {
                    true
                } else {
                    let old_count = prev_keys.len();
                    let new_count = new_keys.len();
                    if old_count >= 10 && new_count >= 10 {
                        let jac = jaccard_similarity(&prev_keys, &new_keys);
                        jac < 0.40
                    } else {
                        // For small lists, treat size spikes as significant.
                        let delta = old_count.abs_diff(new_count);
                        delta >= 25 && (delta as f64) / ((old_count.max(new_count)) as f64) > 0.50
                    }
                }
            };

            let new_len = new_lines.len();
            let max_scroll = new_len.saturating_sub(1);
            if significant && self.scroll > 0 {
                self.scroll = 0;
            } else if prev_lines_len > 0 && new_len > 0 {
                self.scroll = self.scroll.min(max_scroll);
            } else {
                self.scroll = 0;
            }

            self.lines = new_lines;
        } else {
            self.change_summary = ChangeSummary::default();
            self.baseline_compact = None;
            self.change_keys.clear();
            self.lines = lines;
            self.scroll = 0;
        }
        self.updated_at = now_ts();
    }

    pub(in crate::tui_shell) fn remote_repo_missing(&self) -> bool {
        self.ctx == RootContext::Remote
            && self
                .lines
                .iter()
                .any(|l| l.to_lowercase().contains("remote repo not found"))
    }
}

fn sanitize_dashboard_err(msg: &str) -> String {
    const REPO_NOT_FOUND_SUFFIX: &str =
        " (create it with `converge remote create-repo` or POST /repos)";

    let msg = msg.trim();
    let msg = msg.strip_suffix(REPO_NOT_FOUND_SUFFIX).unwrap_or(msg);
    format!("dashboard: {}", msg)
}

impl View for RootView {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn mode(&self) -> UiMode {
        UiMode::Root
    }

    fn title(&self) -> &str {
        match self.ctx {
            RootContext::Local => "Status",
            RootContext::Remote => "Dashboard",
        }
    }

    fn updated_at(&self) -> &str {
        &self.updated_at
    }

    fn move_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    fn move_down(&mut self) {
        if self.scroll < self.lines.len().saturating_sub(1) {
            self.scroll += 1;
        }
    }

    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect, _ctx: &RenderCtx) {
        let inner = match self.ctx {
            RootContext::Local => {
                let title = self.title();

                let baseline = self.baseline_compact.as_deref().unwrap_or("");
                let baseline_prefix = if baseline.is_empty() { "" } else { "  " };

                // Header width heuristic: only show baseline if it fits.
                let a = format!("A:{}", self.change_summary.added);
                let m = format!("M:{}", self.change_summary.modified);
                let d = format!("D:{}", self.change_summary.deleted);
                let r = format!("R:{}", self.change_summary.renamed);
                let base_len = title.len() + 2 + a.len() + 2 + m.len() + 2 + d.len() + 2 + r.len();
                let include_baseline = !baseline.is_empty()
                    && (area.width as usize) >= (base_len + baseline_prefix.len() + baseline.len());

                let header = Line::from(vec![
                    Span::styled(title.to_string(), Style::default().fg(Color::Yellow)),
                    Span::raw("  "),
                    Span::styled(a, Style::default().fg(Color::Green)),
                    Span::raw(" "),
                    Span::styled(m, Style::default().fg(Color::Yellow)),
                    Span::raw(" "),
                    Span::styled(d, Style::default().fg(Color::Red)),
                    Span::raw(" "),
                    Span::styled(r, Style::default().fg(Color::Cyan)),
                    Span::raw(if include_baseline {
                        baseline_prefix
                    } else {
                        ""
                    }),
                    Span::styled(
                        if include_baseline {
                            baseline.to_string()
                        } else {
                            String::new()
                        },
                        Style::default().fg(Color::White),
                    ),
                ]);
                render_view_chrome_with_header(frame, header, area)
            }
            RootContext::Remote => {
                let header = Line::from(vec![
                    Span::styled(
                        self.title().to_string(),
                        Style::default().fg(root_ctx_color(RootContext::Remote)),
                    ),
                    Span::raw("  "),
                    Span::styled(
                        fmt_ts_ui(self.updated_at()),
                        Style::default().fg(Color::Gray),
                    ),
                ]);
                let inner = render_view_chrome_with_header(frame, header, area);
                if let Some(lines) = self.remote_auth_block_lines.as_ref() {
                    frame.render_widget(
                        Paragraph::new(
                            lines
                                .iter()
                                .map(|s| Line::from(s.as_str()))
                                .collect::<Vec<_>>(),
                        )
                        .wrap(Wrap { trim: false }),
                        inner,
                    );
                    return;
                }
                if let Some(d) = &self.remote_dashboard {
                    render_remote_dashboard(frame, inner, d);
                    return;
                }

                // Fallback error rendering.
                let err = self.remote_err.as_deref().unwrap_or("dashboard: error");
                frame.render_widget(
                    Paragraph::new(vec![Line::from(err)])
                        .wrap(Wrap { trim: false })
                        .block(Block::default().borders(Borders::ALL).title("Dashboard")),
                    inner,
                );
                return;
            }
        };

        let mut include_baseline_line = true;
        if self.ctx == RootContext::Local {
            let title = self.title();
            let baseline = self.baseline_compact.as_deref().unwrap_or("");
            if !baseline.is_empty() {
                let a = format!("A:{}", self.change_summary.added);
                let m = format!("M:{}", self.change_summary.modified);
                let d = format!("D:{}", self.change_summary.deleted);
                let r = format!("R:{}", self.change_summary.renamed);
                let base_len = title.len() + 2 + a.len() + 2 + m.len() + 2 + d.len() + 2 + r.len();
                let include_baseline = (area.width as usize) >= (base_len + 2 + baseline.len());
                if include_baseline {
                    include_baseline_line = false;
                }
            }
        }

        let mut lines = Vec::new();
        for s in &self.lines {
            if !include_baseline_line && s.trim_start().starts_with("baseline:") {
                continue;
            }
            lines.push(style_root_line(s));
        }
        if lines.is_empty() {
            lines.push(Line::from(""));
        }

        let scroll = self.scroll.min(lines.len().saturating_sub(1)) as u16;
        frame.render_widget(
            Paragraph::new(lines)
                .wrap(Wrap { trim: false })
                .scroll((scroll, 0)),
            inner,
        );
    }
}

fn render_remote_dashboard(frame: &mut ratatui::Frame, area: Rect, d: &DashboardData) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(0)])
        .split(area);

    // Next actions (top row).
    let mut action_lines: Vec<Line<'static>> = Vec::new();
    if d.next_actions.is_empty() {
        action_lines.push(Line::from("(none)"));
    } else {
        for a in &d.next_actions {
            action_lines.push(Line::from(format!("- {}", a)));
        }
    }
    frame.render_widget(
        Paragraph::new(action_lines)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL).title("Next")),
        rows[0],
    );

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[1]);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(0)])
        .split(cols[0]);
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(0)])
        .split(cols[1]);

    // Inbox.
    let mut inbox_lines: Vec<Line<'static>> = Vec::new();
    inbox_lines.push(Line::from(format!(
        "{} total  {} pending  {} resolved",
        d.inbox_total, d.inbox_pending, d.inbox_resolved
    )));
    if d.inbox_missing_local > 0 {
        inbox_lines.push(Line::from(format!(
            "{} snaps missing locally",
            d.inbox_missing_local
        )));
    }
    if let Some((sid, ts)) = &d.latest_publication {
        inbox_lines.push(Line::from(format!("latest: {} {}", sid, ts)));
    }
    frame.render_widget(
        Paragraph::new(inbox_lines)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL).title("Inbox")),
        left[0],
    );

    // Bundles.
    let mut bundle_lines: Vec<Line<'static>> = Vec::new();
    bundle_lines.push(Line::from(format!(
        "{} total  {} promotable  {} blocked",
        d.bundles_total, d.bundles_promotable, d.bundles_blocked
    )));
    if d.blocked_superpositions > 0 {
        bundle_lines.push(Line::from(format!(
            "blocked by superpositions: {}",
            d.blocked_superpositions
        )));
    }
    if d.blocked_approvals > 0 {
        bundle_lines.push(Line::from(format!(
            "blocked by approvals: {}",
            d.blocked_approvals
        )));
    }
    if d.pinned_bundles > 0 {
        bundle_lines.push(Line::from(format!("pinned: {}", d.pinned_bundles)));
    }
    frame.render_widget(
        Paragraph::new(bundle_lines)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL).title("Bundles")),
        left[1],
    );

    // Gates / scope.
    let mut gate_lines: Vec<Line<'static>> = Vec::new();
    if let Some(h) = &d.healthz {
        gate_lines.push(Line::from(format!("healthz: {}", h)));
    }
    if d.gates_total > 0 {
        let term = d.terminal_gate.as_deref().unwrap_or("-");
        gate_lines.push(Line::from(format!(
            "gates: {} (terminal {})",
            d.gates_total, term
        )));
    }
    if !d.promotion_state.is_empty() {
        gate_lines.push(Line::from("promotion_state:"));
        for (gate, bid) in d.promotion_state.iter().take(4) {
            gate_lines.push(Line::from(format!("{} {}", gate, bid)));
        }
    }
    frame.render_widget(
        Paragraph::new(gate_lines)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL).title("Gates")),
        right[0],
    );

    // Releases.
    let mut rel_lines: Vec<Line<'static>> = Vec::new();
    if d.releases_total == 0 {
        rel_lines.push(Line::from("(none)"));
    } else {
        rel_lines.push(Line::from(format!(
            "{} total ({} channels)",
            d.releases_total, d.releases_channels
        )));
        for (ch, bid, ts) in d.latest_releases.iter() {
            rel_lines.push(Line::from(format!("{} {} {}", ch, bid, ts)));
        }
    }
    frame.render_widget(
        Paragraph::new(rel_lines)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL).title("Releases")),
        right[1],
    );
}

fn style_root_line(s: &str) -> Line<'static> {
    // Style change lines like: "A path (+3 -1)", "R* old -> new (+1 -2)".
    let (main, delta) = if let Some((left, right)) = s.rsplit_once(" (")
        && right.ends_with(')')
    {
        (left, Some(&right[..right.len() - 1]))
    } else {
        (s, None)
    };

    let mut spans: Vec<Span<'static>> = Vec::new();
    let (prefix, rest) = if let Some(r) = main.strip_prefix("R* ") {
        ("R*", r)
    } else if let Some(r) = main.strip_prefix("R ") {
        ("R", r)
    } else if let Some(r) = main.strip_prefix("A ") {
        ("A", r)
    } else if let Some(r) = main.strip_prefix("M ") {
        ("M", r)
    } else if let Some(r) = main.strip_prefix("D ") {
        ("D", r)
    } else {
        ("", main)
    };

    if !prefix.is_empty() {
        let style = match prefix {
            "A" => Style::default().fg(Color::Green),
            "D" => Style::default().fg(Color::Red),
            "M" => Style::default().fg(Color::Yellow),
            "R" | "R*" => Style::default().fg(Color::Cyan),
            _ => Style::default(),
        };
        spans.push(Span::styled(prefix.to_string(), style));
        spans.push(Span::raw(" "));
    }
    spans.push(Span::raw(rest.to_string()));

    if let Some(delta) = delta {
        spans.push(Span::raw(" ("));
        let mut first = true;
        for tok in delta.split_whitespace() {
            if !first {
                spans.push(Span::raw(" "));
            }
            first = false;
            let style = if tok.starts_with('+') {
                Style::default().fg(Color::Green)
            } else if tok.starts_with('-') {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Gray)
            };
            spans.push(Span::styled(tok.to_string(), style));
        }
        spans.push(Span::raw(")"));
    }

    Line::from(spans)
}
