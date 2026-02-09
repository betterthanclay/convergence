use super::*;

impl App {
    pub(in crate::tui_shell::app) fn open_confirm_modal(&mut self, action: PendingAction) {
        let (cmd, context) = match &action {
            PendingAction::Root { root_ctx, cmd } => (cmd.as_str(), root_ctx.label()),
            PendingAction::Mode { mode, cmd } => (cmd.as_str(), mode.prompt()),
        };

        let cmd_display = match &action {
            PendingAction::Mode { mode, cmd }
                if *mode == UiMode::Settings && cmd.as_str() == "do" =>
            {
                match self
                    .current_view::<SettingsView>()
                    .and_then(|v| v.selected_kind())
                {
                    Some(SettingsItemKind::ChunkingReset) => "reset chunking".to_string(),
                    Some(SettingsItemKind::RetentionReset) => "reset retention".to_string(),
                    _ => "settings action".to_string(),
                }
            }
            _ => cmd.to_string(),
        };

        let mut lines = Vec::new();
        lines.push(format!("Run: {}", cmd_display));
        lines.push(format!("Where: {}", context));
        lines.push("".to_string());
        lines.push("This action changes data.".to_string());
        lines.push("Enter: confirm    Esc: cancel".to_string());

        self.modal = Some(Modal {
            title: "Confirm".to_string(),
            lines,
            scroll: 0,
            kind: ModalKind::ConfirmAction { action },
            input: Input::default(),
        });
    }
}
