use super::super::TextInputAction;
use super::super::views::{BundlesView, InboxView};
use super::types::{BrowseTarget, BrowseWizard};

impl super::super::App {
    pub(in crate::tui_shell) fn start_browse_wizard(&mut self, target: BrowseTarget) {
        let cfg = match self.remote_config() {
            Some(c) => c,
            None => {
                self.start_login_wizard();
                return;
            }
        };

        let (scope, gate, filter, limit) = match target {
            BrowseTarget::Inbox => self
                .current_view::<InboxView>()
                .map(|v| (v.scope.clone(), v.gate.clone(), v.filter.clone(), v.limit))
                .unwrap_or((cfg.scope.clone(), cfg.gate.clone(), None, None)),
            BrowseTarget::Bundles => self
                .current_view::<BundlesView>()
                .map(|v| (v.scope.clone(), v.gate.clone(), v.filter.clone(), v.limit))
                .unwrap_or((cfg.scope.clone(), cfg.gate.clone(), None, None)),
        };

        self.browse_wizard = Some(BrowseWizard {
            target,
            scope,
            gate,
            filter,
            limit,
        });

        let initial = self.browse_wizard.as_ref().map(|w| w.scope.clone());
        self.open_text_input_modal(
            "Browse",
            "scope> ",
            TextInputAction::BrowseScope,
            initial,
            vec!["Scope id (Enter keeps current).".to_string()],
        );
    }

    pub(in crate::tui_shell) fn continue_browse_wizard(
        &mut self,
        action: TextInputAction,
        value: String,
    ) {
        if self.browse_wizard.is_none() {
            self.push_error("browse wizard not active".to_string());
            return;
        }

        match action {
            TextInputAction::BrowseScope => {
                let v = value.trim().to_string();
                if let Some(w) = self.browse_wizard.as_mut()
                    && !v.is_empty()
                {
                    w.scope = v;
                }
                let initial = self.browse_wizard.as_ref().map(|w| w.gate.clone());
                self.open_text_input_modal(
                    "Browse",
                    "gate> ",
                    TextInputAction::BrowseGate,
                    initial,
                    vec!["Gate id (Enter keeps current).".to_string()],
                );
            }
            TextInputAction::BrowseGate => {
                let v = value.trim().to_string();
                if let Some(w) = self.browse_wizard.as_mut()
                    && !v.is_empty()
                {
                    w.gate = v;
                }
                let initial = self.browse_wizard.as_ref().and_then(|w| w.filter.clone());
                self.open_text_input_modal(
                    "Browse",
                    "filter (blank=none)> ",
                    TextInputAction::BrowseFilter,
                    initial,
                    vec!["Optional filter query".to_string()],
                );
            }
            TextInputAction::BrowseFilter => {
                let v = value.trim().to_string();
                if let Some(w) = self.browse_wizard.as_mut() {
                    w.filter = if v.is_empty() { None } else { Some(v) };
                }
                let initial = self
                    .browse_wizard
                    .as_ref()
                    .and_then(|w| w.limit)
                    .map(|n| n.to_string());
                self.open_text_input_modal(
                    "Browse",
                    "limit (blank=none)> ",
                    TextInputAction::BrowseLimit,
                    initial,
                    vec!["Optional limit".to_string()],
                );
            }
            TextInputAction::BrowseLimit => {
                let v = value.trim().to_string();
                let limit = if v.is_empty() {
                    None
                } else {
                    match v.parse::<usize>() {
                        Ok(n) => Some(n),
                        Err(_) => {
                            self.open_text_input_modal(
                                "Browse",
                                "limit (blank=none)> ",
                                TextInputAction::BrowseLimit,
                                Some(v),
                                vec!["error: invalid number".to_string()],
                            );
                            return;
                        }
                    }
                };
                if let Some(w) = self.browse_wizard.as_mut() {
                    w.limit = limit;
                }
                self.finish_browse_wizard();
            }
            _ => {
                self.push_error("unexpected browse wizard input".to_string());
            }
        }
    }

    pub(in crate::tui_shell) fn finish_browse_wizard(&mut self) {
        let Some(w) = self.browse_wizard.clone() else {
            self.push_error("browse wizard not active".to_string());
            return;
        };
        self.browse_wizard = None;

        match w.target {
            BrowseTarget::Inbox => self.open_inbox_view(w.scope, w.gate, w.filter, w.limit),
            BrowseTarget::Bundles => self.open_bundles_view(w.scope, w.gate, w.filter, w.limit),
        }
    }
}
