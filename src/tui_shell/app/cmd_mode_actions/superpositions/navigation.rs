use super::*;

impl App {
    pub(in crate::tui_shell) fn cmd_superpositions_pick_mode(&mut self, args: &[String]) {
        if args.len() != 1 {
            self.push_error("usage: pick <n>".to_string());
            return;
        }
        let n = match args[0].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                self.push_error("invalid variant number".to_string());
                return;
            }
        };
        if n == 0 {
            self.push_error("variant numbers are 1-based".to_string());
            return;
        }
        super::superpositions_nav::superpositions_pick_variant(self, n - 1);
    }

    pub(in crate::tui_shell) fn cmd_superpositions_clear_mode(&mut self, args: &[String]) {
        if !args.is_empty() {
            self.push_error("usage: clear".to_string());
            return;
        }
        super::superpositions_nav::superpositions_clear_decision(self);
    }

    pub(in crate::tui_shell) fn cmd_superpositions_next_missing_mode(&mut self, args: &[String]) {
        if !args.is_empty() {
            self.push_error("usage: next-missing".to_string());
            return;
        }
        super::superpositions_nav::superpositions_jump_next_missing(self);
    }

    pub(in crate::tui_shell) fn cmd_superpositions_next_invalid_mode(&mut self, args: &[String]) {
        if !args.is_empty() {
            self.push_error("usage: next-invalid".to_string());
            return;
        }
        super::superpositions_nav::superpositions_jump_next_invalid(self);
    }
}
