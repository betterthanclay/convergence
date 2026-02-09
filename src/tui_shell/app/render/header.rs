use super::*;

pub(super) fn render_header(frame: &mut ratatui::Frame, app: &App, area: ratatui::layout::Rect) {
    let header_mid = if app.root_ctx == RootContext::Remote {
        app.workspace
            .as_ref()
            .and_then(|ws| ws.store.read_config().ok())
            .and_then(|c| c.remote)
            .map(|r| format!("repo={} scope={} gate={}", r.repo_id, r.scope, r.gate))
            .unwrap_or_else(|| "(no remote configured)".to_string())
    } else {
        app.workspace
            .as_ref()
            .map(|w| w.root.display().to_string())
            .or_else(|| app.workspace_err.clone())
            .unwrap_or_else(|| "(no workspace)".to_string())
    };

    let mut spans = vec![
        Span::styled(
            "Converge",
            Style::default().fg(Color::Black).bg(Color::White),
        ),
        Span::raw("  "),
        Span::styled(
            app.prompt(),
            Style::default().fg(root_ctx_color(app.root_ctx)),
        ),
        Span::raw("  "),
        Span::raw(header_mid),
    ];
    if let Some(id) = app.remote_identity.as_deref() {
        spans.push(Span::raw("  "));
        spans.push(Span::styled(id, Style::default().fg(Color::Green)));
    } else if let Some(note) = app.remote_identity_note.as_deref() {
        spans.push(Span::raw("  "));
        spans.push(Span::styled(note, Style::default().fg(Color::Red)));
    }

    let header = Paragraph::new(Line::from(spans)).block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(header, area);
}
