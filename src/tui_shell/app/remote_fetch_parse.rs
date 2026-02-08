#[derive(Debug, Default)]
pub(super) struct FetchSpec {
    pub(super) snap_id: Option<String>,
    pub(super) bundle_id: Option<String>,
    pub(super) release: Option<String>,
    pub(super) lane: Option<String>,
    pub(super) user: Option<String>,
    pub(super) restore: bool,
    pub(super) into: Option<String>,
    pub(super) force: bool,
}

pub(super) fn parse_fetch_spec(args: &[String]) -> Result<FetchSpec, String> {
    let mut out = FetchSpec::default();

    // Flagless UX:
    // - `fetch snap <id>`
    // - `fetch bundle <id> [restore] [into <dir>] [force]`
    // - `fetch release <channel> [restore] [into <dir>] [force]`
    // - `fetch lane <lane> [user <handle>]`
    // - `fetch <snap_id>` (shorthand)
    let mut free = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--snap-id" | "snap" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    return Err("usage: fetch (snap|bundle|release|lane) <id...>".to_string());
                };
                out.snap_id = Some(v.clone());
            }
            "--bundle-id" | "bundle" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    return Err("usage: fetch (snap|bundle|release|lane) <id...>".to_string());
                };
                out.bundle_id = Some(v.clone());
            }
            "--release" | "release" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    return Err("usage: fetch (snap|bundle|release|lane) <id...>".to_string());
                };
                out.release = Some(v.clone());
            }
            "--lane" | "lane" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    return Err("usage: fetch (snap|bundle|release|lane) <id...>".to_string());
                };
                out.lane = Some(v.clone());
            }
            "--user" | "user" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    return Err("usage: fetch lane <lane> [user <handle>]".to_string());
                };
                out.user = Some(v.clone());
            }
            "--restore" | "restore" => {
                out.restore = true;
            }
            "--into" | "into" => {
                i += 1;
                let Some(v) = args.get(i) else {
                    return Err("usage: fetch [restore] [into <dir>] [force]".to_string());
                };
                out.into = Some(v.clone());
            }
            "--force" | "force" => {
                out.force = true;
            }
            a => {
                free.push(a.to_string());
            }
        }
        i += 1;
    }

    // Allow `fetch <snap_id>` shorthand.
    if !free.is_empty()
        && out.snap_id.is_none()
        && out.bundle_id.is_none()
        && out.release.is_none()
        && out.lane.is_none()
        && out.user.is_none()
        && free.len() == 1
    {
        out.snap_id = Some(free[0].clone());
        free.clear();
    }

    // Allow `fetch lane <lane> <user>` shorthand.
    if !free.is_empty() && out.lane.is_some() && out.user.is_none() && free.len() == 1 {
        out.user = Some(free[0].clone());
        free.clear();
    }

    if !free.is_empty() {
        return Err("usage: fetch (snap|bundle|release|lane) <id...>".to_string());
    }

    if (out.bundle_id.is_some() || out.release.is_some())
        && (out.snap_id.is_some() || out.lane.is_some() || out.user.is_some())
    {
        return Err("fetch: choose one target: snap/lane, or bundle, or release".to_string());
    }

    if out.bundle_id.is_some() && out.release.is_some() {
        return Err("fetch: choose one target: bundle or release".to_string());
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
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
}
