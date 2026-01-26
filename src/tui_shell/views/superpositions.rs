use std::any::Any;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use crate::model::{ObjectId, ResolutionDecision};
use crate::resolve::ResolutionValidation;

use super::super::{RenderCtx, UiMode, View, render_view_chrome};

#[derive(Debug)]
pub(in crate::tui_shell) struct SuperpositionsView {
    pub(in crate::tui_shell) updated_at: String,
    pub(in crate::tui_shell) bundle_id: String,
    pub(in crate::tui_shell) filter: Option<String>,
    pub(in crate::tui_shell) root_manifest: ObjectId,
    pub(in crate::tui_shell) variants:
        std::collections::BTreeMap<String, Vec<crate::model::SuperpositionVariant>>,
    pub(in crate::tui_shell) decisions: std::collections::BTreeMap<String, ResolutionDecision>,
    pub(in crate::tui_shell) validation: Option<ResolutionValidation>,
    pub(in crate::tui_shell) items: Vec<(String, usize)>,
    pub(in crate::tui_shell) selected: usize,
}

impl View for SuperpositionsView {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn mode(&self) -> UiMode {
        UiMode::Superpositions
    }

    fn title(&self) -> &str {
        "Superpositions"
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

    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect, _ctx: &RenderCtx) {
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
        for (p, n) in &self.items {
            let mark = match self.decisions.get(p) {
                None => " ".to_string(),
                Some(ResolutionDecision::Index(i)) => {
                    let n = (*i as usize) + 1;
                    if n <= 9 {
                        format!("{}", n)
                    } else {
                        "*".to_string()
                    }
                }
                Some(ResolutionDecision::Key(k)) => {
                    let idx = self
                        .variants
                        .get(p)
                        .and_then(|vs| vs.iter().position(|v| v.key() == *k));
                    match idx {
                        Some(i) if i < 9 => format!("{}", i + 1),
                        Some(_) => "*".to_string(),
                        None => "!".to_string(),
                    }
                }
            };
            rows.push(ListItem::new(format!("[{}] {} ({})", mark, p, n)));
        }
        if rows.is_empty() {
            rows.push(ListItem::new("(none)"));
        }

        let list = List::new(rows)
            .block(Block::default().borders(Borders::BOTTOM).title(format!(
                "bundle={}{}{} (pick; Alt+1..9, Alt+0; / for commands)",
                self.bundle_id.chars().take(8).collect::<String>(),
                self.filter
                    .as_ref()
                    .map(|f| format!(" filter={}", f))
                    .unwrap_or_default(),
                self.validation
                    .as_ref()
                    .map(|v| {
                        format!(
                            " missing={} invalid={}",
                            v.missing.len(),
                            v.invalid_keys.len() + v.out_of_range.len()
                        )
                    })
                    .unwrap_or_default()
            )))
            .highlight_style(Style::default().bg(Color::DarkGray));
        frame.render_stateful_widget(list, parts[0], &mut state);

        let details = if self.items.is_empty() {
            vec![Line::from("(no selection)")]
        } else {
            let idx = self.selected.min(self.items.len().saturating_sub(1));
            let (p, n) = &self.items[idx];
            let mut out = Vec::new();
            out.push(Line::from(format!("path: {}", p)));
            out.push(Line::from(format!("variants: {}", n)));
            out.push(Line::from(format!(
                "root_manifest: {}",
                self.root_manifest.as_str()
            )));

            if let Some(vr) = &self.validation {
                out.push(Line::from(""));
                out.push(Line::from(format!(
                    "validation: {}",
                    if vr.ok { "ok" } else { "invalid" }
                )));
                if !vr.missing.is_empty() {
                    out.push(Line::from(format!("missing: {}", vr.missing.len())));
                }
                if !vr.invalid_keys.is_empty() {
                    out.push(Line::from(format!(
                        "invalid_keys: {}",
                        vr.invalid_keys.len()
                    )));
                }
                if !vr.out_of_range.is_empty() {
                    out.push(Line::from(format!(
                        "out_of_range: {}",
                        vr.out_of_range.len()
                    )));
                }
                if !vr.extraneous.is_empty() {
                    out.push(Line::from(format!("extraneous: {}", vr.extraneous.len())));
                }
            }

            let chosen = self.decisions.get(p);
            out.push(Line::from(""));
            match chosen {
                None => {
                    out.push(Line::from("decision: (missing)"));
                }
                Some(ResolutionDecision::Index(i)) => {
                    out.push(Line::from(format!("decision: index {}", i)));
                }
                Some(ResolutionDecision::Key(k)) => {
                    let key_json = serde_json::to_string(k).unwrap_or_else(|_| "<key>".to_string());
                    out.push(Line::from(format!("decision: key {}", key_json)));
                }
            }

            if let Some(vs) = self.variants.get(p) {
                out.push(Line::from(""));
                out.push(Line::from("variants:"));
                for (i, v) in vs.iter().enumerate() {
                    let key_json =
                        serde_json::to_string(&v.key()).unwrap_or_else(|_| "<key>".to_string());
                    out.push(Line::from(format!("  #{} source={}", i + 1, v.source)));
                    out.push(Line::from(format!("    key={}", key_json)));
                    match &v.kind {
                        crate::model::SuperpositionVariantKind::File { blob, mode, size } => {
                            out.push(Line::from(format!(
                                "    file blob={} mode={:#o} size={}",
                                blob.as_str(),
                                mode,
                                size
                            )));
                        }
                        crate::model::SuperpositionVariantKind::FileChunks {
                            recipe,
                            mode,
                            size,
                        } => {
                            out.push(Line::from(format!(
                                "    chunked_file recipe={} mode={:#o} size={}",
                                recipe.as_str(),
                                mode,
                                size
                            )));
                        }
                        crate::model::SuperpositionVariantKind::Dir { manifest } => {
                            out.push(Line::from(format!(
                                "    dir manifest={} ",
                                manifest.as_str()
                            )));
                        }
                        crate::model::SuperpositionVariantKind::Symlink { target } => {
                            out.push(Line::from(format!("    symlink target={}", target)));
                        }
                        crate::model::SuperpositionVariantKind::Tombstone => {
                            out.push(Line::from("    tombstone"));
                        }
                    }
                }
            }

            out
        };
        frame.render_widget(Paragraph::new(details).wrap(Wrap { trim: false }), parts[1]);
    }
}
