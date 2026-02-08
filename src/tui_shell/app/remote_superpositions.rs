use super::remote_action_parse::parse_superpositions_args;
use super::*;

impl App {
    pub(super) fn cmd_superpositions(&mut self, args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };
        let client = match self.remote_client() {
            Some(c) => c,
            None => {
                self.start_login_wizard();
                return;
            }
        };

        if args.is_empty() {
            self.open_text_input_modal(
                "Superpositions",
                "bundle id> ",
                TextInputAction::SuperpositionsBundleId,
                None,
                vec!["Bundle id".to_string()],
            );
            return;
        }

        let parsed = match parse_superpositions_args(args) {
            Ok(p) => p,
            Err(msg) => {
                self.push_error(msg);
                return;
            }
        };
        let Some(bundle_id) = parsed.bundle_id else {
            self.push_error("usage: superpositions <bundle_id>".to_string());
            return;
        };

        let bundle = match client.get_bundle(&bundle_id) {
            Ok(b) => b,
            Err(err) => {
                self.push_error(format!("get bundle: {:#}", err));
                return;
            }
        };

        let root = crate::model::ObjectId(bundle.root_manifest.clone());
        if let Err(err) = client.fetch_manifest_tree(&ws.store, &root) {
            self.push_error(format!("fetch manifest tree: {:#}", err));
            return;
        }

        let variants = match superposition_variants(&ws.store, &root) {
            Ok(v) => v,
            Err(err) => {
                self.push_error(format!("scan superpositions: {:#}", err));
                return;
            }
        };

        let mut decisions = std::collections::BTreeMap::new();
        if ws.store.has_resolution(&bundle_id)
            && let Ok(r) = ws.store.get_resolution(&bundle_id)
            && r.root_manifest == root
        {
            decisions = r.decisions;
        }

        let validation = validate_resolution(&ws.store, &root, &decisions).ok();

        let filter_lc = parsed.filter.as_ref().map(|s| s.to_lowercase());
        let mut items = variants
            .iter()
            .map(|(p, vs)| (p.clone(), vs.len()))
            .collect::<Vec<_>>();
        items.sort_by(|a, b| a.0.cmp(&b.0));
        if let Some(q) = filter_lc.as_deref() {
            items.retain(|(p, _)| p.to_lowercase().contains(q));
        }

        let count = items.len();
        self.push_view(SuperpositionsView {
            updated_at: now_ts(),
            bundle_id,
            filter: parsed.filter,
            root_manifest: root,
            variants,
            decisions,
            validation,
            items,
            selected: 0,
        });
        self.push_output(vec![format!("opened superpositions ({} paths)", count)]);
    }
}
