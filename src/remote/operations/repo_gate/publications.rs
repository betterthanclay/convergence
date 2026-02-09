use super::*;

impl RemoteClient {
    pub fn list_publications(&self) -> Result<Vec<Publication>> {
        let repo = &self.remote.repo_id;
        let resp = self
            .client
            .get(self.url(&format!("/repos/{}/publications", repo)))
            .header(reqwest::header::AUTHORIZATION, self.auth())
            .send()
            .context("list publications")?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            anyhow::bail!(
                "remote repo not found (create it with `converge remote create-repo` or POST /repos)"
            );
        }

        let pubs: Vec<Publication> = self
            .ensure_ok(resp, "list publications")?
            .json()
            .context("parse publications")?;
        Ok(pubs)
    }
}
