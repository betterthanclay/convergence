use super::*;

impl App {
    pub(in crate::tui_shell::app) fn dispatch_global(
        &mut self,
        cmd: &str,
        args: &[String],
    ) -> bool {
        match cmd {
            "quit" => {
                self.quit = true;
                true
            }
            "settings" => {
                self.cmd_settings(args);
                true
            }
            "login" => {
                if self.mode() != UiMode::Root {
                    self.push_error("login is only available at root".to_string());
                } else {
                    self.cmd_login(args);
                }
                true
            }
            "logout" => {
                if self.mode() != UiMode::Root {
                    self.push_error("logout is only available at root".to_string());
                } else {
                    self.cmd_logout(args);
                }
                true
            }
            _ => false,
        }
    }
}
