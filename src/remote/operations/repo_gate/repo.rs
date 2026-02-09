use super::*;

impl RemoteClient {
    pub fn create_repo(&self, repo_id: &str) -> Result<Repo> {
        let resp = self
            .client
            .post(self.url("/repos"))
            .header(reqwest::header::AUTHORIZATION, self.auth())
            .json(&CreateRepoRequest {
                id: repo_id.to_string(),
            })
            .send()
            .context("create repo request")?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            anyhow::bail!("remote endpoint not found (is converge-server running?)");
        }

        let resp = self.ensure_ok(resp, "create repo")?;
        let repo: Repo = resp.json().context("parse create repo response")?;
        Ok(repo)
    }
}
