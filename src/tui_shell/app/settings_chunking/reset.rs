use super::*;

pub(super) fn handle_reset_chunking(app: &mut App, ws: &Workspace) {
    let mut cfg = match ws.store.read_config() {
        Ok(c) => c,
        Err(err) => {
            app.push_error(format!("read config: {:#}", err));
            return;
        }
    };
    cfg.chunking = None;
    if let Err(err) = ws.store.write_config(&cfg) {
        app.push_error(format!("write config: {:#}", err));
        return;
    }
    app.refresh_root_view();
    app.push_output(vec!["reset chunking config".to_string()]);
}
