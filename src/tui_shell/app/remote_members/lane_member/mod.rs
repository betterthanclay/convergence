use super::*;

mod legacy_flags;
mod prompt_first;

impl App {
    pub(in crate::tui_shell::app) fn cmd_lane_member(&mut self, args: &[String]) {
        if args.is_empty() {
            self.start_lane_member_wizard(None);
            return;
        }

        if prompt_first::is_prompt_first_subcommand(args[0].as_str()) {
            prompt_first::handle_prompt_first(self, args);
            return;
        }

        legacy_flags::handle_legacy_flags(self, args);
    }
}
