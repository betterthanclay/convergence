use super::move_glob::glob_search;

mod prompts;
mod source;
mod target;

impl super::super::App {
    pub(super) fn move_wizard_from(&mut self, value: String) {
        source::move_wizard_from(self, value);
    }

    pub(super) fn move_wizard_to(&mut self, value: String) {
        target::move_wizard_to(self, value);
    }
}
