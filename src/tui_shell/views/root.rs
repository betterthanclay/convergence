use std::any::Any;

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::workspace::Workspace;

use super::super::app::{now_ts, root_ctx_color};
use super::super::status::{ChangeSummary, DashboardData, dashboard_data, local_status_lines};
use super::super::view::render_view_chrome_with_header;
use super::super::{RenderCtx, RootContext, UiMode, View, fmt_ts_ui};

mod local_header;
mod refresh_local;
mod render_remote;
mod style_line;

use self::local_header::local_header_and_baseline_line;
use self::refresh_local::{clear_local_tracking_for_remote, refresh_local_state};
use self::render_remote::render_remote_dashboard;
use self::style_line::style_root_line;

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
            refresh_local_state(self, lines, prev_lines_len, prev_baseline, prev_keys);
        } else {
            clear_local_tracking_for_remote(self, lines);
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
        let (inner, include_baseline_line) = match self.ctx {
            RootContext::Local => {
                let (header, keep_baseline_line) = local_header_and_baseline_line(self, area.width);
                (
                    render_view_chrome_with_header(frame, header, area),
                    keep_baseline_line,
                )
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
