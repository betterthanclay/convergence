use super::restore::temp_restore_path;
use super::*;

pub(super) fn fetch_bundle(
    app: &mut App,
    client: &RemoteClient,
    ws: &Workspace,
    parsed: &FetchSpec,
    bundle_id: &str,
) {
    let bundle = match client.get_bundle(bundle_id) {
        Ok(b) => b,
        Err(err) => {
            app.push_error(format!("get bundle: {:#}", err));
            return;
        }
    };
    let root = crate::model::ObjectId(bundle.root_manifest.clone());
    if let Err(err) = client.fetch_manifest_tree(&ws.store, &root) {
        app.push_error(format!("fetch bundle objects: {:#}", err));
        return;
    }

    if parsed.restore {
        let dest = parsed
            .into
            .as_deref()
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                let short = bundle.id.chars().take(8).collect::<String>();
                temp_restore_path("bundle", &short)
            });
        if let Err(err) = ws.materialize_manifest_to(&root, &dest, parsed.force) {
            app.push_error(format!("restore: {:#}", err));
            return;
        }
        app.push_output(vec![format!(
            "materialized bundle {} into {}",
            bundle.id,
            dest.display()
        )]);
    } else {
        app.push_output(vec![format!("fetched bundle {}", bundle.id)]);
    }
    app.refresh_root_view();
}

pub(super) fn fetch_release(
    app: &mut App,
    client: &RemoteClient,
    ws: &Workspace,
    parsed: &FetchSpec,
    channel: &str,
) {
    let rel = match client.get_release(channel) {
        Ok(r) => r,
        Err(err) => {
            app.push_error(format!("get release: {:#}", err));
            return;
        }
    };
    let bundle = match client.get_bundle(&rel.bundle_id) {
        Ok(b) => b,
        Err(err) => {
            app.push_error(format!("get bundle: {:#}", err));
            return;
        }
    };

    let root = crate::model::ObjectId(bundle.root_manifest.clone());
    if let Err(err) = client.fetch_manifest_tree(&ws.store, &root) {
        app.push_error(format!("fetch release objects: {:#}", err));
        return;
    }

    if parsed.restore {
        let dest = parsed
            .into
            .as_deref()
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                let short = rel.bundle_id.chars().take(8).collect::<String>();
                temp_restore_path("release", &short)
            });
        if let Err(err) = ws.materialize_manifest_to(&root, &dest, parsed.force) {
            app.push_error(format!("restore: {:#}", err));
            return;
        }
        app.push_output(vec![format!(
            "materialized release {} ({}) into {}",
            rel.channel,
            rel.bundle_id,
            dest.display()
        )]);
    } else {
        app.push_output(vec![format!(
            "fetched release {} ({})",
            rel.channel, rel.bundle_id
        )]);
    }
    app.refresh_root_view();
}
