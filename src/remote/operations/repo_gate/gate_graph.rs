use super::validation::format_gate_graph_validation_error;
use super::*;

impl RemoteClient {
    pub fn get_gate_graph(&self) -> Result<GateGraph> {
        let repo = &self.remote.repo_id;
        let resp = self
            .client
            .get(self.url(&format!("/repos/{}/gate-graph", repo)))
            .header(reqwest::header::AUTHORIZATION, self.auth())
            .send()
            .context("get gate graph")?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            anyhow::bail!(
                "remote repo not found (create it with `converge remote create-repo` or POST /repos)"
            );
        }

        let graph: GateGraph = self
            .ensure_ok(resp, "get gate graph")?
            .json()
            .context("parse gate graph")?;
        Ok(graph)
    }

    pub fn put_gate_graph(&self, graph: &GateGraph) -> Result<GateGraph> {
        let repo = &self.remote.repo_id;
        let resp = self
            .client
            .put(self.url(&format!("/repos/{}/gate-graph", repo)))
            .header(reqwest::header::AUTHORIZATION, self.auth())
            .json(graph)
            .send()
            .context("put gate graph")?;

        if resp.status() == reqwest::StatusCode::BAD_REQUEST {
            let v: GateGraphValidationError =
                resp.json().context("parse gate graph validation error")?;
            anyhow::bail!(format_gate_graph_validation_error(&v));
        }
        let graph: GateGraph = self
            .ensure_ok(resp, "put gate graph")?
            .json()
            .context("parse gate graph")?;
        Ok(graph)
    }
}
