use super::*;

impl RemoteClient {
    pub fn update_lane_head_me(
        &self,
        lane_id: &str,
        snap_id: &str,
        client_id: Option<String>,
    ) -> Result<LaneHead> {
        let repo = &self.remote.repo_id;
        let resp = self
            .client
            .post(self.url(&format!("/repos/{}/lanes/{}/heads/me", repo, lane_id)))
            .header(reqwest::header::AUTHORIZATION, self.auth())
            .json(&UpdateLaneHeadRequest {
                snap_id: snap_id.to_string(),
                client_id,
            })
            .send()
            .context("update lane head")?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            anyhow::bail!("remote lane not found (check `converge lanes` or /repos/:repo/lanes)");
        }

        let head: LaneHead = self
            .ensure_ok(resp, "update lane head")?
            .json()
            .context("parse lane head")?;
        Ok(head)
    }

    pub fn get_lane_head(&self, lane_id: &str, user: &str) -> Result<LaneHead> {
        let repo = &self.remote.repo_id;
        let resp = self
            .client
            .get(self.url(&format!("/repos/{}/lanes/{}/heads/{}", repo, lane_id, user)))
            .header(reqwest::header::AUTHORIZATION, self.auth())
            .send()
            .context("get lane head")?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            anyhow::bail!("lane head not found");
        }

        let head: LaneHead = self
            .ensure_ok(resp, "get lane head")?
            .json()
            .context("parse lane head")?;
        Ok(head)
    }
}
