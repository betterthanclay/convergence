use anyhow::{Context, Result};

use crate::model::SnapRecord;
use crate::remote::CreatePublicationRequest;

use super::{Publication, PublicationResolution, RemoteClient, with_retries};

pub(super) fn create_publication(
    client: &RemoteClient,
    repo: &str,
    snap: &SnapRecord,
    scope: &str,
    gate: &str,
    metadata_only: bool,
    resolution: Option<PublicationResolution>,
) -> Result<Publication> {
    let resp = with_retries("create publication", || {
        let resp = client
            .client
            .post(client.url(&format!("/repos/{}/publications", repo)))
            .header(reqwest::header::AUTHORIZATION, client.auth())
            .json(&CreatePublicationRequest {
                snap_id: snap.id.clone(),
                scope: scope.to_string(),
                gate: gate.to_string(),
                metadata_only,
                resolution: resolution.clone(),
            })
            .send()
            .context("send")?;
        client.ensure_ok(resp, "create publication")
    })?;

    resp.json().context("parse publication")
}
