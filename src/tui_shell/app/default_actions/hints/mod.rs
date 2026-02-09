use super::*;

mod mode_hints;
mod rotation;

impl App {
    pub(in crate::tui_shell::app) fn rotate_hint(&mut self, dir: i32) {
        rotation::rotate_hint(self, dir);
    }

    pub(in crate::tui_shell::app) fn primary_hint_commands(&self) -> Vec<String> {
        let raw = mode_hints::hint_commands_raw(self);
        if raw.is_empty() {
            return raw;
        }
        let n = raw.len();
        let rot = self.hint_rotation[rotation::hint_key(self)] % n;
        if rot == 0 {
            return raw;
        }
        raw.into_iter().cycle().skip(rot).take(n).collect()
    }
}
