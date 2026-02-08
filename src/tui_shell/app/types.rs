#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tui_shell) enum UiMode {
    Root,
    Snaps,
    Inbox,
    Bundles,
    Releases,
    Lanes,
    Superpositions,
    GateGraph,
    Settings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tui_shell) enum RootContext {
    Local,
    Remote,
}

impl RootContext {
    pub(in crate::tui_shell) fn label(self) -> &'static str {
        match self {
            RootContext::Local => "local",
            RootContext::Remote => "remote",
        }
    }
}

impl UiMode {
    pub(in crate::tui_shell) fn prompt(self) -> &'static str {
        match self {
            UiMode::Root => "root>",
            UiMode::Snaps => "history>",
            UiMode::Inbox => "inbox>",
            UiMode::Bundles => "bundles>",
            UiMode::Releases => "releases>",
            UiMode::Lanes => "lanes>",
            UiMode::Superpositions => "supers>",
            UiMode::GateGraph => "gates>",
            UiMode::Settings => "settings>",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tui_shell) enum TimestampMode {
    Relative,
    Absolute,
}

impl TimestampMode {
    pub(in crate::tui_shell) fn toggle(self) -> Self {
        match self {
            TimestampMode::Relative => TimestampMode::Absolute,
            TimestampMode::Absolute => TimestampMode::Relative,
        }
    }

    pub(in crate::tui_shell) fn label(self) -> &'static str {
        match self {
            TimestampMode::Relative => "relative",
            TimestampMode::Absolute => "absolute",
        }
    }
}
