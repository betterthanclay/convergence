use super::*;

impl RemoteClient {
    pub fn list_repo_members(&self) -> Result<RepoMembers> {
        let repo = &self.remote.repo_id;
        let resp = self
            .client
            .get(self.url(&format!("/repos/{}/members", repo)))
            .header(reqwest::header::AUTHORIZATION, self.auth())
            .send()
            .context("list repo members")?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            anyhow::bail!("remote repo not found");
        }

        let out: RepoMembers = self
            .ensure_ok(resp, "list repo members")?
            .json()
            .context("parse repo members")?;
        Ok(out)
    }

    pub fn add_repo_member(&self, handle: &str, role: &str) -> Result<()> {
        let repo = &self.remote.repo_id;
        let resp = self
            .client
            .post(self.url(&format!("/repos/{}/members", repo)))
            .header(reqwest::header::AUTHORIZATION, self.auth())
            .json(&serde_json::json!({"handle": handle, "role": role}))
            .send()
            .context("add repo member")?;

        let _ = self.ensure_ok(resp, "add repo member")?;
        Ok(())
    }

    pub fn remove_repo_member(&self, handle: &str) -> Result<()> {
        let repo = &self.remote.repo_id;
        let resp = self
            .client
            .delete(self.url(&format!("/repos/{}/members/{}", repo, handle)))
            .header(reqwest::header::AUTHORIZATION, self.auth())
            .send()
            .context("remove repo member")?;
        let _ = self.ensure_ok(resp, "remove repo member")?;
        Ok(())
    }
}
