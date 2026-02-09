use super::super::super::*;

pub(super) fn handle_alt_shortcut(app: &mut App, c: char) {
    if app.mode() != UiMode::Superpositions {
        return;
    }

    if c.is_ascii_digit() {
        let n = c.to_digit(10).unwrap_or(0) as usize;
        if n == 0 {
            superpositions_nav::superpositions_clear_decision(app);
        } else {
            superpositions_nav::superpositions_pick_variant(app, n - 1);
        }
    }

    if c == 'f' {
        superpositions_nav::superpositions_jump_next_invalid(app);
    }

    if c == 'n' {
        superpositions_nav::superpositions_jump_next_missing(app);
    }
}
