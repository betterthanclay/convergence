use super::*;

pub(super) fn finish_bootstrap_wizard(app: &mut super::super::super::App) {
    let Some(w) = app.bootstrap_wizard.clone() else {
        app.push_error("bootstrap wizard not active".to_string());
        return;
    };
    app.bootstrap_wizard = None;

    let (base_url, bootstrap_token, handle, repo_id) = match parse_bootstrap_inputs(&w) {
        Ok(inputs) => (
            inputs.base_url,
            inputs.bootstrap_token,
            inputs.handle,
            inputs.repo_id,
        ),
        Err(err) => {
            app.push_error(err);
            return;
        }
    };

    let remote = RemoteConfig {
        base_url: base_url.clone(),
        token: None,
        repo_id: repo_id.clone(),
        scope: w.scope.clone(),
        gate: w.gate.clone(),
    };

    let client = match crate::remote::RemoteClient::new(remote.clone(), bootstrap_token) {
        Ok(c) => c,
        Err(err) => {
            app.push_error(format!("bootstrap: {:#}", err));
            return;
        }
    };

    let bootstrap = match client.bootstrap_first_admin(&handle, w.display_name.clone()) {
        Ok(r) => r,
        Err(err) => {
            app.push_error(format!("bootstrap: {:#}", err));
            return;
        }
    };

    app.apply_login_config(
        base_url.clone(),
        bootstrap.token.token.clone(),
        repo_id.clone(),
        w.scope.clone(),
        w.gate.clone(),
    );

    repo::ensure_repo_exists(app, &repo_id);

    app.push_output(vec![
        format!("bootstrapped admin {}", bootstrap.user.handle),
        "Restart the server without --bootstrap-token.".to_string(),
    ]);
    app.refresh_root_view();
}
