use super::*;

pub(super) fn prune_release_history(
    repo: &mut Repo,
    keep_last: Option<usize>,
) -> Result<usize, Response> {
    let Some(keep_last) = keep_last else {
        return Ok(0);
    };
    if keep_last == 0 {
        return Err(bad_request(anyhow::anyhow!(
            "prune_releases_keep_last must be >= 1"
        )));
    }

    let releases_before = repo.releases.len();
    let mut by_channel: HashMap<String, Vec<Release>> = HashMap::new();
    for release in repo.releases.clone() {
        by_channel
            .entry(release.channel.clone())
            .or_default()
            .push(release);
    }

    let mut kept: Vec<Release> = Vec::new();
    for (_channel, mut releases) in by_channel {
        releases.sort_by(|a, b| b.released_at.cmp(&a.released_at));
        releases.truncate(keep_last);
        kept.extend(releases);
    }
    kept.sort_by(|a, b| b.released_at.cmp(&a.released_at));

    let pruned_count = releases_before.saturating_sub(kept.len());
    repo.releases = kept;
    Ok(pruned_count)
}
