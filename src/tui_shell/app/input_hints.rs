use super::*;

pub(super) fn input_hint_left(app: &App) -> Option<String> {
    if !app.input.buf.is_empty() {
        return None;
    }
    if app.modal.is_some() {
        return None;
    }

    let cmds = app.primary_hint_commands();
    if cmds.is_empty() {
        return None;
    }

    Some(cmds.join(" | "))
}

pub(super) fn input_hint_right(app: &App) -> Option<(Line<'static>, usize)> {
    if !app.input.buf.is_empty() {
        return None;
    }
    if app.modal.is_some() {
        return None;
    }
    if app.mode() != UiMode::Root {
        return None;
    }

    let tab_target = match app.root_ctx {
        RootContext::Local => ("remote", Color::Blue),
        RootContext::Remote => ("local", Color::Yellow),
    };

    Some((
        Line::from(vec![
            Span::styled(
                "/".to_string(),
                Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
            ),
            Span::raw(" cmds  "),
            Span::styled(
                "Enter".to_string(),
                Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
            ),
            Span::raw(" default  "),
            Span::styled(
                "Esc".to_string(),
                Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
            ),
            Span::raw(" back  "),
            Span::styled(
                "q".to_string(),
                Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
            ),
            Span::raw(" quit  "),
            Span::styled(
                "Tab".to_string(),
                Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
            ),
            Span::raw(":"),
            Span::styled(tab_target.0.to_string(), Style::default().fg(tab_target.1)),
        ]),
        format!(
            "/ cmds  Enter default  Esc back  q quit  Tab:{}",
            tab_target.0
        )
        .len(),
    ))
}
