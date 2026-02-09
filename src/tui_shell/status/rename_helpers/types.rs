#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(in crate::tui_shell::status) enum IdentityKey {
    Blob(String),
    Recipe(String),
    Symlink(String),
}

#[derive(Clone, Debug)]
pub(in crate::tui_shell::status) enum StatusChange {
    Added(String),
    Modified(String),
    Deleted(String),
    Renamed {
        from: String,
        to: String,
        modified: bool,
    },
}

impl StatusChange {
    pub(in crate::tui_shell::status) fn sort_key(&self) -> (&str, &str) {
        match self {
            StatusChange::Added(p) => ("A", p.as_str()),
            StatusChange::Modified(p) => ("M", p.as_str()),
            StatusChange::Deleted(p) => ("D", p.as_str()),
            StatusChange::Renamed { from, .. } => ("R", from.as_str()),
        }
    }
}
