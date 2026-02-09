pub(super) fn ensure_repo_exists(app: &mut super::super::super::App, repo_id: &str) {
    // Ensure the repo exists for the configured remote (best-effort).
    if let Some(client) = app.remote_client() {
        match client.get_repo(repo_id) {
            Ok(_) => {
                app.push_output(vec![format!("repo {} exists", repo_id)]);
            }
            Err(err) if err.to_string().contains("remote repo not found") => {
                match client.create_repo(repo_id) {
                    Ok(_) => app.push_output(vec![format!("created repo {}", repo_id)]),
                    Err(err) => app.push_error(format!("create repo: {:#}", err)),
                }
            }
            Err(err) => {
                app.push_error(format!("get repo: {:#}", err));
            }
        }
    }
}
