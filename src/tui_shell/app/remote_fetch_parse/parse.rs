use super::FetchSpec;

const TARGET_USAGE: &str = "usage: fetch (snap|bundle|release|lane) <id...>";
const LANE_USAGE: &str = "usage: fetch lane <lane> [user <handle>]";
const RESTORE_USAGE: &str = "usage: fetch [restore] [into <dir>] [force]";

pub(super) struct ParsedFetchSpec {
    pub(super) spec: FetchSpec,
    pub(super) free: Vec<String>,
}

pub(super) fn parse_tokens(args: &[String]) -> Result<ParsedFetchSpec, String> {
    let mut spec = FetchSpec::default();
    let mut free: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--snap-id" | "snap" => {
                i += 1;
                spec.snap_id = Some(required(args, i, TARGET_USAGE)?);
            }
            "--bundle-id" | "bundle" => {
                i += 1;
                spec.bundle_id = Some(required(args, i, TARGET_USAGE)?);
            }
            "--release" | "release" => {
                i += 1;
                spec.release = Some(required(args, i, TARGET_USAGE)?);
            }
            "--lane" | "lane" => {
                i += 1;
                spec.lane = Some(required(args, i, TARGET_USAGE)?);
            }
            "--user" | "user" => {
                i += 1;
                spec.user = Some(required(args, i, LANE_USAGE)?);
            }
            "--restore" | "restore" => spec.restore = true,
            "--into" | "into" => {
                i += 1;
                spec.into = Some(required(args, i, RESTORE_USAGE)?);
            }
            "--force" | "force" => spec.force = true,
            other => free.push(other.to_string()),
        }
        i += 1;
    }

    Ok(ParsedFetchSpec { spec, free })
}

pub(super) fn apply_shorthands(parsed: &mut ParsedFetchSpec) -> Result<(), String> {
    if parsed.spec.lane.is_some()
        && parsed.spec.user.is_none()
        && parsed.free.len() == 1
        && let Some(free) = parsed.free.pop()
    {
        parsed.spec.user = Some(free);
    }

    if parsed.spec.snap_id.is_none()
        && parsed.spec.bundle_id.is_none()
        && parsed.spec.release.is_none()
        && parsed.spec.lane.is_none()
        && parsed.spec.user.is_none()
        && parsed.free.len() == 1
        && let Some(free) = parsed.free.pop()
    {
        parsed.spec.snap_id = Some(free);
    }

    if !parsed.free.is_empty() {
        return Err(TARGET_USAGE.to_string());
    }
    Ok(())
}

fn required(args: &[String], index: usize, usage: &str) -> Result<String, String> {
    args.get(index).cloned().ok_or_else(|| usage.to_string())
}
