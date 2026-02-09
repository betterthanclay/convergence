use super::*;
use crate::model::ObjectId;

pub(super) fn build_resolved_snap(
    app: &mut App,
    ws: &Workspace,
    bundle_id: &str,
    root_manifest: &ObjectId,
) -> std::result::Result<crate::model::SnapRecord, ()> {
    let resolution = match ws.store.get_resolution(bundle_id) {
        Ok(r) => r,
        Err(err) => {
            app.push_error(format!("load resolution: {:#}", err));
            return Err(());
        }
    };
    if resolution.root_manifest != *root_manifest {
        app.push_error("resolution root_manifest mismatch".to_string());
        return Err(());
    }

    let resolved_root =
        match crate::resolve::apply_resolution(&ws.store, root_manifest, &resolution.decisions) {
            Ok(r) => r,
            Err(err) => {
                app.push_error(format!("apply resolution: {:#}", err));
                return Err(());
            }
        };

    let created_at = now_ts();
    let snap_id = crate::model::compute_snap_id(&created_at, &resolved_root);
    Ok(crate::model::SnapRecord {
        version: 1,
        id: snap_id,
        created_at: created_at.clone(),
        root_manifest: resolved_root,
        message: None,
        stats: crate::model::SnapStats::default(),
    })
}
