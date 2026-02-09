use super::*;

mod header;
mod input;
mod status;
mod suggestions;

use self::header::render_header;
use self::input::{render_input, render_right_hint, set_cursor};
use self::status::render_status;
use self::suggestions::render_suggestions;

pub(super) fn draw(frame: &mut ratatui::Frame, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(if app.suggestions.is_empty() { 0 } else { 9 }),
            Constraint::Length(3),
        ])
        .split(area);

    render_header(frame, app, chunks[0]);

    let ctx = RenderCtx {
        now: OffsetDateTime::now_utc(),
        ts_mode: app.ts_mode,
    };
    app.view().render(frame, chunks[1], &ctx);

    render_status(frame, app, chunks[2]);

    if !app.suggestions.is_empty() {
        render_suggestions(frame, app, chunks[3]);
    }

    render_input(frame, app, chunks[4]);
    render_right_hint(frame, app, chunks[4]);

    if let Some(m) = &app.modal {
        dim_frame(frame);
        modal::draw_modal(frame, m);
        return;
    }

    set_cursor(frame, app, chunks[4]);
}

fn dim_frame(frame: &mut ratatui::Frame) {
    let area = frame.area();
    let buf = frame.buffer_mut();
    for y in area.y..area.y.saturating_add(area.height) {
        for x in area.x..area.x.saturating_add(area.width) {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.modifier |= Modifier::DIM;
            }
        }
    }
}
