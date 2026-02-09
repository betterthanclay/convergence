use super::super::super::*;

pub(super) fn handle_input_edit_keys(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Backspace => {
            app.input.backspace();
            app.recompute_suggestions();
            true
        }
        KeyCode::Delete => {
            app.input.delete();
            app.recompute_suggestions();
            true
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input.clear();
            app.recompute_suggestions();
            true
        }
        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input.history_up();
            app.recompute_suggestions();
            true
        }
        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input.history_down();
            app.recompute_suggestions();
            true
        }
        _ => false,
    }
}
