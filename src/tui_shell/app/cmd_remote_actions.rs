use super::remote_action_parse::parse_bundle_args;
use super::*;

impl App {
    pub(super) fn cmd_bundle(&mut self, args: &[String]) {
        if args.is_empty() {
            self.cmd_inbox(&[]);
            self.push_output(vec![
                "opened inbox for bundling".to_string(),
                "tip: select a publication, then use `bundle` (or rotate hints then Enter)"
                    .to_string(),
            ]);
            return;
        }

        let client = match self.remote_client() {
            Some(c) => c,
            None => {
                self.start_login_wizard();
                return;
            }
        };
        let cfg = match self.remote_config() {
            Some(c) => c,
            None => {
                self.start_login_wizard();
                return;
            }
        };

        let parsed = match parse_bundle_args(args) {
            Ok(v) => v,
            Err(msg) => {
                self.push_error(msg);
                return;
            }
        };
        let scope = parsed.scope.unwrap_or(cfg.scope);
        let gate = parsed.gate.unwrap_or(cfg.gate);
        let mut pubs = parsed.publications;

        if pubs.is_empty() {
            let all = match client.list_publications() {
                Ok(p) => p,
                Err(err) => {
                    self.push_error(format!("list publications: {:#}", err));
                    return;
                }
            };
            pubs = all
                .into_iter()
                .filter(|p| p.scope == scope && p.gate == gate)
                .map(|p| p.id)
                .collect();
        }

        if pubs.is_empty() {
            self.push_error("no publications to bundle".to_string());
            return;
        }

        match client.create_bundle(&scope, &gate, &pubs) {
            Ok(b) => self.push_output(vec![format!("bundle {}", b.id)]),
            Err(err) => self.push_error(format!("bundle: {:#}", err)),
        }
    }

    pub(super) fn cmd_pins(&mut self, args: &[String]) {
        let _ = args;
        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };

        match client.list_pins() {
            Ok(mut pins) => {
                pins.bundles.sort();
                let mut out = Vec::new();
                out.push(format!("pinned bundles: {}", pins.bundles.len()));
                out.extend(pins.bundles);
                self.push_output(out);
                self.refresh_root_view();
            }
            Err(err) => {
                self.push_error(format!("pins: {:#}", err));
            }
        }
    }
}
