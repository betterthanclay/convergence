use super::super::TextInputAction;
use super::super::views::{BundlesView, InboxView};
use super::types::{BrowseTarget, BrowseWizard};
use crate::tui_shell::App;

impl App {
    pub(in crate::tui_shell) fn start_browse_wizard(&mut self, target: BrowseTarget) {
        start::start_browse_wizard(self, target);
    }

    pub(in crate::tui_shell) fn continue_browse_wizard(
        &mut self,
        action: TextInputAction,
        value: String,
    ) {
        transitions::continue_browse_wizard(self, action, value);
    }

    pub(in crate::tui_shell) fn finish_browse_wizard(&mut self) {
        finish::finish_browse_wizard(self);
    }
}

mod finish;
mod start;
mod transitions;
