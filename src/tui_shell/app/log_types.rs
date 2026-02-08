#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tui_shell::app) enum EntryKind {
    Command,
    Output,
    Error,
}

#[derive(Clone, Debug)]
pub(in crate::tui_shell::app) struct ScrollEntry {
    pub(in crate::tui_shell::app) ts: String,
    pub(in crate::tui_shell::app) kind: EntryKind,
    pub(in crate::tui_shell::app) lines: Vec<String>,
}

#[derive(Clone, Debug)]
pub(in crate::tui_shell) struct CommandDef {
    pub(in crate::tui_shell) name: &'static str,
    pub(in crate::tui_shell) aliases: &'static [&'static str],
    pub(in crate::tui_shell) usage: &'static str,
    pub(in crate::tui_shell) help: &'static str,
}
