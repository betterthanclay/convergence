use super::*;

mod pinning;
mod set_reset;
mod show;

impl App {
    pub(super) fn cmd_retention(&mut self, args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };

        let sub = args.first().map(|s| s.as_str()).unwrap_or("show");
        match sub {
            "show" => show::show_retention(self, &ws),
            "set" => set_reset::set_retention(self, &ws, args),
            "reset" => set_reset::reset_retention(self, &ws),
            "pin" | "unpin" => pinning::pin_or_unpin_retention(self, &ws, sub, args),
            _ => self.push_error(
                "usage: settings retention show | settings retention set [--keep-last N] [--keep-days N] [--prune-snaps true|false] | settings retention pin <snap> | settings retention unpin <snap> | settings retention reset"
                    .to_string(),
            ),
        }
    }
}
