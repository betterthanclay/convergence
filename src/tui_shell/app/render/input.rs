use super::*;

pub(super) fn render_input(frame: &mut ratatui::Frame, app: &App, area: ratatui::layout::Rect) {
    let prompt = app.prompt();
    let buf = &app.input.buf;
    let prompt_color = root_ctx_color(app.root_ctx);

    let mut input_spans = Vec::new();
    input_spans.push(Span::styled(prompt, Style::default().fg(prompt_color)));
    input_spans.push(Span::raw(" "));
    input_spans.push(Span::raw(buf.as_str()));

    if let Some(hint) = input_hint_left(app) {
        let sep = if buf.is_empty() { "" } else { "  " };
        input_spans.push(Span::raw(sep));
        input_spans.push(Span::styled(
            hint,
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
        ));
    }

    let input =
        Paragraph::new(Line::from(input_spans)).block(Block::default().borders(Borders::TOP));
    frame.render_widget(input, area);
}

pub(super) fn render_right_hint(
    frame: &mut ratatui::Frame,
    app: &App,
    area: ratatui::layout::Rect,
) {
    if let Some((hint_line, hint_len)) = input_hint_right(app) {
        let inner_w = area.width.saturating_sub(2) as usize;
        let left_len = app.prompt().len() + 1 + app.input.buf.len();
        let left_hint_len = input_hint_left(app)
            .map(|h| (if app.input.buf.is_empty() { 0 } else { 2 }) + h.len())
            .unwrap_or(0);
        let right_len = hint_len;
        if left_len + left_hint_len + 1 + right_len <= inner_w {
            let rect = ratatui::layout::Rect {
                x: area.x + 1,
                y: area.y + 1,
                width: area.width.saturating_sub(2),
                height: 1,
            };
            frame.render_widget(
                Paragraph::new(hint_line).alignment(ratatui::layout::Alignment::Right),
                rect,
            );
        }
    }
}

pub(super) fn set_cursor(frame: &mut ratatui::Frame, app: &App, area: ratatui::layout::Rect) {
    let x = app.prompt().len() as u16 + 1 + app.input.cursor as u16;
    let y = area.y + 1;
    frame.set_cursor_position((area.x + x, y));
}
