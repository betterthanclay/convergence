use super::*;

pub(super) fn render_status(frame: &mut ratatui::Frame, app: &App, area: ratatui::layout::Rect) {
    let mut lines = Vec::new();
    if let Some(cmd) = &app.last_command {
        lines.push(Line::from(vec![
            Span::styled("> ", Style::default().fg(Color::Cyan)),
            Span::raw(cmd.as_str()),
        ]));
    }
    if let Some(r) = &app.last_result {
        let style = match r.kind {
            EntryKind::Output => Style::default().fg(Color::White),
            EntryKind::Error => Style::default().fg(Color::Red),
            EntryKind::Command => Style::default().fg(Color::Cyan),
        };
        for (i, l) in r.lines.iter().enumerate() {
            if i == 0 {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("{} ", fmt_ts_ui(&r.ts)),
                        Style::default().fg(Color::Gray),
                    ),
                    Span::styled(l.as_str(), style),
                ]));
            } else {
                lines.push(Line::from(Span::styled(l.as_str(), style)));
            }
        }
    }
    if lines.is_empty() {
        lines.push(Line::from(""));
    }
    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::TOP).title("Last")),
        area,
    );
}
