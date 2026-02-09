use anyhow::Result;

use super::*;

pub(super) fn collect(
    client: &RemoteClient,
    remote: &crate::model::RemoteConfig,
    out: &mut DashboardData,
) -> Result<()> {
    let mut bundles = client.list_bundles()?;
    bundles.retain(|b| b.scope == remote.scope && b.gate == remote.gate);
    out.bundles_total = bundles.len();
    out.bundles_promotable = bundles.iter().filter(|b| b.promotable).count();
    out.bundles_blocked = out.bundles_total.saturating_sub(out.bundles_promotable);
    for bundle in &bundles {
        if bundle.promotable {
            continue;
        }
        if bundle
            .reasons
            .iter()
            .any(|reason| reason == "superpositions_present")
        {
            out.blocked_superpositions += 1;
        }
        if bundle
            .reasons
            .iter()
            .any(|reason| reason == "approvals_missing")
        {
            out.blocked_approvals += 1;
        }
    }
    if let Ok(pins) = client.list_pins() {
        out.pinned_bundles = pins.bundles.len();
    }

    if let Ok(state) = client.promotion_state(&remote.scope) {
        let mut keys = state.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        for gate in keys {
            let bid = state.get(&gate).cloned().unwrap_or_default();
            let short = bid.chars().take(8).collect::<String>();
            out.promotion_state.push((gate, short));
        }
    }
    Ok(())
}
