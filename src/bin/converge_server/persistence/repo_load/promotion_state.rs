use super::super::super::*;

pub(super) fn rebuild_promotion_state(
    promotions: &[Promotion],
) -> HashMap<String, HashMap<String, String>> {
    let mut tmp: HashMap<String, HashMap<String, (String, String)>> = HashMap::new();
    for p in promotions {
        let scope_entry = tmp.entry(p.scope.clone()).or_default();
        match scope_entry.get(&p.to_gate) {
            None => {
                scope_entry.insert(
                    p.to_gate.clone(),
                    (p.promoted_at.clone(), p.bundle_id.clone()),
                );
            }
            Some((prev_time, _prev_bundle)) => {
                if p.promoted_at > *prev_time {
                    scope_entry.insert(
                        p.to_gate.clone(),
                        (p.promoted_at.clone(), p.bundle_id.clone()),
                    );
                }
            }
        }
    }

    tmp.into_iter()
        .map(|(scope, m)| {
            let m = m
                .into_iter()
                .map(|(to_gate, (_t, bundle_id))| (to_gate, bundle_id))
                .collect::<HashMap<_, _>>();
            (scope, m)
        })
        .collect()
}
