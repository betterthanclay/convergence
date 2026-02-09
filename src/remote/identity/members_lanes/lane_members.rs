use super::*;

impl RemoteClient {
    pub fn list_lane_members(&self, lane_id: &str) -> Result<LaneMembers> {
        let repo = &self.remote.repo_id;
        let resp = self
            .client
            .get(self.url(&format!("/repos/{}/lanes/{}/members", repo, lane_id)))
            .header(reqwest::header::AUTHORIZATION, self.auth())
            .send()
            .context("list lane members")?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            anyhow::bail!("remote lane not found");
        }

        let out: LaneMembers = self
            .ensure_ok(resp, "list lane members")?
            .json()
            .context("parse lane members")?;
        Ok(out)
    }

    pub fn add_lane_member(&self, lane_id: &str, handle: &str) -> Result<()> {
        let repo = &self.remote.repo_id;
        let resp = self
            .client
            .post(self.url(&format!("/repos/{}/lanes/{}/members", repo, lane_id)))
            .header(reqwest::header::AUTHORIZATION, self.auth())
            .json(&serde_json::json!({"handle": handle}))
            .send()
            .context("add lane member")?;

        let _ = self.ensure_ok(resp, "add lane member")?;
        Ok(())
    }

    pub fn remove_lane_member(&self, lane_id: &str, handle: &str) -> Result<()> {
        let repo = &self.remote.repo_id;
        let resp = self
            .client
            .delete(self.url(&format!(
                "/repos/{}/lanes/{}/members/{}",
                repo, lane_id, handle
            )))
            .header(reqwest::header::AUTHORIZATION, self.auth())
            .send()
            .context("remove lane member")?;
        let _ = self.ensure_ok(resp, "remove lane member")?;
        Ok(())
    }

    pub fn list_lanes(&self) -> Result<Vec<Lane>> {
        let repo = &self.remote.repo_id;
        let resp = self
            .client
            .get(self.url(&format!("/repos/{}/lanes", repo)))
            .header(reqwest::header::AUTHORIZATION, self.auth())
            .send()
            .context("list lanes")?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            anyhow::bail!(
                "remote repo not found (create it with `converge remote create-repo` or POST /repos)"
            );
        }

        let lanes: Vec<Lane> = self
            .ensure_ok(resp, "list lanes")?
            .json()
            .context("parse lanes")?;
        Ok(lanes)
    }
}
