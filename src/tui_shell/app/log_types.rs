#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tui_shell) enum EntryKind {
    Command,
    Output,
    Error,
}

#[derive(Clone, Debug)]
pub(in crate::tui_shell) struct ScrollEntry {
    pub(in crate::tui_shell) ts: String,
    pub(in crate::tui_shell) kind: EntryKind,
    pub(in crate::tui_shell) lines: Vec<String>,
}

#[derive(Clone, Debug)]
pub(in crate::tui_shell) struct CommandDef {
    pub(in crate::tui_shell) name: &'static str,
    pub(in crate::tui_shell) aliases: &'static [&'static str],
    pub(in crate::tui_shell) usage: &'static str,
    pub(in crate::tui_shell) help: &'static str,
}
