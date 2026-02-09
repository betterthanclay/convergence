use super::FetchSpec;

pub(super) fn validate_target_selection(spec: &FetchSpec) -> Result<(), String> {
    if (spec.bundle_id.is_some() || spec.release.is_some())
        && (spec.snap_id.is_some() || spec.lane.is_some() || spec.user.is_some())
    {
        return Err("fetch: choose one target: snap/lane, or bundle, or release".to_string());
    }

    if spec.bundle_id.is_some() && spec.release.is_some() {
        return Err("fetch: choose one target: bundle or release".to_string());
    }

    Ok(())
}
