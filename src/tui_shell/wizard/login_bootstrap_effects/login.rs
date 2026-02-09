use super::*;

pub(super) fn apply_login_config(
    app: &mut super::super::super::App,
    base_url: String,
    token: String,
    repo_id: String,
    scope: String,
    gate: String,
) {
    let Some(ws) = app.require_workspace() else {
        return;
    };

    let mut cfg = match ws.store.read_config() {
        Ok(c) => c,
        Err(err) => {
            app.push_error(format!("read config: {:#}", err));
            return;
        }
    };

    let remote = RemoteConfig {
        base_url: base_url.clone(),
        token: None,
        repo_id,
        scope,
        gate,
    };

    if let Err(err) = ws.store.set_remote_token(&remote, &token) {
        app.push_error(format!("store remote token: {:#}", err));
        return;
    }

    cfg.remote = Some(remote);
    if let Err(err) = ws.store.write_config(&cfg) {
        app.push_error(format!("write config: {:#}", err));
        return;
    }

    app.push_output(vec![format!("logged in to {}", base_url)]);
    app.refresh_root_view();
}
