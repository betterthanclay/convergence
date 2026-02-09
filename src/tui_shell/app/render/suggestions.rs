use super::*;

pub(super) fn render_suggestions(
    frame: &mut ratatui::Frame,
    app: &App,
    area: ratatui::layout::Rect,
) {
    let mut s_lines = Vec::new();
    let total = app.suggestions.len();
    let sel_idx = app
        .suggestion_selected
        .min(app.suggestions.len().saturating_sub(1));
    s_lines.push(Line::from(Span::styled(
        format!("Suggestions {}/{}", sel_idx + 1, total),
        Style::default().fg(Color::Gray),
    )));

    let inner_h = area.height.saturating_sub(2) as usize;
    let max_items = inner_h.saturating_sub(1).max(1);
    let mut start = 0usize;
    if total > max_items {
        if sel_idx >= max_items {
            start = sel_idx + 1 - max_items;
        }
        start = start.min(total.saturating_sub(max_items));
    }
    let end = (start + max_items).min(total);

    for i in start..end {
        let s = &app.suggestions[i];
        let sel = i == sel_idx;
        let style = if sel {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };
        s_lines.push(Line::from(vec![
            Span::styled(format!("{: <10}", s.name), style.fg(Color::Yellow)),
            Span::styled(s.help, style.fg(Color::White)),
        ]));
    }
    let sugg =
        Paragraph::new(s_lines).block(Block::default().borders(Borders::TOP | Borders::BOTTOM));
    frame.render_widget(sugg, area);
}
