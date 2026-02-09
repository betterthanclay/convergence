use super::*;

impl App {
    pub(in crate::tui_shell::app) fn cmd_chunking(&mut self, args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };

        let sub = args.first().map(|s| s.as_str()).unwrap_or("show");
        match sub {
            "show" => show_chunking(self, &ws),
            "set" => set::handle_set_chunking(self, &ws, args),
            "reset" => reset::handle_reset_chunking(self, &ws),
            _ => {
                self.push_error(
                    "usage: settings chunking show | settings chunking set --chunk-size-mib N --threshold-mib N | settings chunking reset"
                        .to_string(),
                );
            }
        }
    }
}

fn show_chunking(app: &mut App, ws: &Workspace) {
    let cfg = match ws.store.read_config() {
        Ok(c) => c,
        Err(err) => {
            app.push_error(format!("read config: {:#}", err));
            return;
        }
    };

    let (chunk_size, threshold) = cfg
        .chunking
        .as_ref()
        .map(|c| (c.chunk_size, c.threshold))
        .unwrap_or((4 * 1024 * 1024, 8 * 1024 * 1024));
    let lines = vec![
        format!("chunk_size: {} MiB", chunk_size / (1024 * 1024)),
        format!("threshold: {} MiB", threshold / (1024 * 1024)),
        "".to_string(),
        "Files with size >= threshold are stored as chunked files.".to_string(),
    ];
    app.open_modal("Chunking", lines);
}
