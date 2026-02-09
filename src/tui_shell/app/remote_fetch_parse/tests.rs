use super::parse_fetch_spec;

fn argv(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

#[test]
fn parses_single_free_arg_as_snap_id() {
    let parsed = parse_fetch_spec(&argv(&["snap-123"])).expect("parse should succeed");
    assert_eq!(parsed.snap_id.as_deref(), Some("snap-123"));
    assert!(parsed.bundle_id.is_none());
    assert!(parsed.release.is_none());
    assert!(parsed.lane.is_none());
}

#[test]
fn parses_bundle_restore_into_force_flow() {
    let parsed = parse_fetch_spec(&argv(&[
        "bundle", "bun-1", "restore", "into", "/tmp/out", "force",
    ]))
    .expect("parse should succeed");

    assert_eq!(parsed.bundle_id.as_deref(), Some("bun-1"));
    assert!(parsed.restore);
    assert_eq!(parsed.into.as_deref(), Some("/tmp/out"));
    assert!(parsed.force);
}

#[test]
fn parses_lane_with_trailing_user_shorthand() {
    let parsed =
        parse_fetch_spec(&argv(&["lane", "review", "alice"])).expect("parse should succeed");
    assert_eq!(parsed.lane.as_deref(), Some("review"));
    assert_eq!(parsed.user.as_deref(), Some("alice"));
}

#[test]
fn rejects_bundle_and_release_together() {
    let err = parse_fetch_spec(&argv(&["bundle", "bun-1", "release", "stable"]))
        .expect_err("parse should fail");
    assert!(
        err.contains("choose one target: bundle or release"),
        "unexpected error: {err}"
    );
}

#[test]
fn rejects_bundle_mixed_with_snap_target() {
    let err = parse_fetch_spec(&argv(&["bundle", "bun-1", "snap", "snap-1"]))
        .expect_err("parse should fail");
    assert!(
        err.contains("choose one target: snap/lane, or bundle, or release"),
        "unexpected error: {err}"
    );
}
