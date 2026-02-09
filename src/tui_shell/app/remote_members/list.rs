use super::*;

impl App {
    pub(in crate::tui_shell::app) fn cmd_members(&mut self, args: &[String]) {
        let _ = args;
        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };

        let members = match client.list_repo_members() {
            Ok(m) => m,
            Err(err) => {
                self.push_error(format!("members: {:#}", err));
                return;
            }
        };

        let lanes = client.list_lanes().ok();

        let mut lines = Vec::new();
        lines.push("Repo".to_string());
        lines.push(format!("owner: {}", members.owner));

        let publishers: std::collections::HashSet<String> =
            members.publishers.iter().cloned().collect();
        let mut readers = members.readers;
        readers.sort();
        lines.push("".to_string());
        lines.push("members:".to_string());
        for h in readers {
            let role = if publishers.contains(&h) {
                "publish"
            } else {
                "read"
            };
            lines.push(format!("- {} {}", h, role));
        }

        if let Some(mut lanes) = lanes {
            lanes.sort_by(|a, b| a.id.cmp(&b.id));
            lines.push("".to_string());
            lines.push("Lanes".to_string());
            for l in lanes {
                let mut m = l.members.into_iter().collect::<Vec<_>>();
                m.sort();
                lines.push(format!("lane {} ({})", l.id, m.len()));
                if !m.is_empty() {
                    let preview = m.into_iter().take(10).collect::<Vec<_>>().join(", ");
                    lines.push(format!("  {}", preview));
                }
            }
        }

        lines.push("".to_string());
        lines.push("hint: type `member` or `lane-member`".to_string());
        self.open_modal("Members", lines);
    }
}
