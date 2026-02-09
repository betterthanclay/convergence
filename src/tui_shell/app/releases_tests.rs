use super::*;

fn mk_release(
    id: &str,
    channel: &str,
    bundle_id: &str,
    released_at: &str,
) -> crate::remote::Release {
    crate::remote::Release {
        id: id.to_string(),
        channel: channel.to_string(),
        bundle_id: bundle_id.to_string(),
        scope: "main".to_string(),
        gate: "dev-intake".to_string(),
        released_by: "dev".to_string(),
        released_by_user_id: None,
        released_at: released_at.to_string(),
        notes: None,
    }
}

#[test]
fn latest_releases_by_channel_picks_latest_and_sorts() {
    let out = latest_releases_by_channel(vec![
        mk_release("r1", "stable", "b1", "2026-01-25T00:00:00Z"),
        mk_release("r2", "stable", "b2", "2026-01-25T01:00:00Z"),
        mk_release("r3", "beta", "b3", "2026-01-25T00:30:00Z"),
    ]);
    assert_eq!(out.len(), 2);
    assert_eq!(out[0].channel, "beta");
    assert_eq!(out[0].bundle_id, "b3");
    assert_eq!(out[1].channel, "stable");
    assert_eq!(out[1].bundle_id, "b2");
}
