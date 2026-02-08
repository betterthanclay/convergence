use crate::tui_shell::CommandDef;

pub(in crate::tui_shell) fn snaps_command_defs() -> Vec<CommandDef> {
    vec![
        CommandDef {
            name: "back",
            aliases: &[],
            usage: "back",
            help: "Return to root",
        },
        CommandDef {
            name: "filter",
            aliases: &[],
            usage: "filter <q>",
            help: "Filter snaps by id/message/time",
        },
        CommandDef {
            name: "clear-filter",
            aliases: &["unfilter"],
            usage: "clear-filter",
            help: "Clear snap filter",
        },
        CommandDef {
            name: "snap",
            aliases: &[],
            usage: "snap [message...]",
            help: "Create a snap from pending changes",
        },
        CommandDef {
            name: "msg",
            aliases: &[],
            usage: "msg [message...] | msg clear",
            help: "Set/clear message on selected snap",
        },
        CommandDef {
            name: "revert",
            aliases: &[],
            usage: "revert",
            help: "Revert pending changes back to head (confirm)",
        },
        CommandDef {
            name: "unsnap",
            aliases: &[],
            usage: "unsnap",
            help: "Delete head snap while keeping the workspace state (confirm)",
        },
        CommandDef {
            name: "restore",
            aliases: &[],
            usage: "restore [<snap>] [force]",
            help: "Restore selected snap",
        },
    ]
}

pub(in crate::tui_shell) fn inbox_command_defs() -> Vec<CommandDef> {
    vec![
        CommandDef {
            name: "back",
            aliases: &[],
            usage: "back",
            help: "Return to root",
        },
        CommandDef {
            name: "edit",
            aliases: &[],
            usage: "edit",
            help: "Edit scope/gate/filter/limit",
        },
        CommandDef {
            name: "bundle",
            aliases: &[],
            usage: "bundle [<publication_id>]",
            help: "Create bundle from selection",
        },
        CommandDef {
            name: "fetch",
            aliases: &[],
            usage: "fetch [<snap_id>]",
            help: "Fetch selected snap",
        },
    ]
}

pub(in crate::tui_shell) fn bundles_command_defs() -> Vec<CommandDef> {
    vec![
        CommandDef {
            name: "back",
            aliases: &[],
            usage: "back",
            help: "Return to root",
        },
        CommandDef {
            name: "edit",
            aliases: &[],
            usage: "edit",
            help: "Edit scope/gate/filter/limit",
        },
        CommandDef {
            name: "approve",
            aliases: &[],
            usage: "approve [<bundle_id>]",
            help: "Approve selected bundle",
        },
        CommandDef {
            name: "pin",
            aliases: &[],
            usage: "pin [unpin]",
            help: "Pin/unpin selected bundle",
        },
        CommandDef {
            name: "promote",
            aliases: &[],
            usage: "promote [to <gate>]",
            help: "Promote selected bundle",
        },
        CommandDef {
            name: "release",
            aliases: &[],
            usage: "release",
            help: "Create a release from selected bundle",
        },
        CommandDef {
            name: "superpositions",
            aliases: &["supers"],
            usage: "superpositions",
            help: "Open superpositions for selected bundle",
        },
    ]
}

pub(in crate::tui_shell) fn releases_command_defs() -> Vec<CommandDef> {
    vec![
        CommandDef {
            name: "back",
            aliases: &[],
            usage: "back",
            help: "Return to root",
        },
        CommandDef {
            name: "fetch",
            aliases: &[],
            usage: "fetch [restore] [into <dir>] [force]",
            help: "Fetch selected release (optional restore)",
        },
    ]
}

pub(in crate::tui_shell) fn lanes_command_defs() -> Vec<CommandDef> {
    vec![
        CommandDef {
            name: "back",
            aliases: &[],
            usage: "back",
            help: "Return to root",
        },
        CommandDef {
            name: "fetch",
            aliases: &[],
            usage: "fetch",
            help: "Fetch selected lane head",
        },
    ]
}

pub(in crate::tui_shell) fn gate_graph_command_defs() -> Vec<CommandDef> {
    vec![
        CommandDef {
            name: "back",
            aliases: &[],
            usage: "back",
            help: "Return to root",
        },
        CommandDef {
            name: "refresh",
            aliases: &["r"],
            usage: "refresh",
            help: "Reload gate graph from server",
        },
        CommandDef {
            name: "add-gate",
            aliases: &[],
            usage: "add-gate",
            help: "Add a new gate (guided)",
        },
        CommandDef {
            name: "remove-gate",
            aliases: &[],
            usage: "remove-gate",
            help: "Remove selected gate (confirm)",
        },
        CommandDef {
            name: "edit-upstream",
            aliases: &[],
            usage: "edit-upstream",
            help: "Edit upstream list (guided)",
        },
        CommandDef {
            name: "set-approvals",
            aliases: &[],
            usage: "set-approvals",
            help: "Set required approvals (guided)",
        },
        CommandDef {
            name: "toggle-releases",
            aliases: &[],
            usage: "toggle-releases",
            help: "Toggle allow_releases",
        },
        CommandDef {
            name: "toggle-superpositions",
            aliases: &[],
            usage: "toggle-superpositions",
            help: "Toggle allow_superpositions",
        },
        CommandDef {
            name: "toggle-metadata-only",
            aliases: &[],
            usage: "toggle-metadata-only",
            help: "Toggle allow_metadata_only_publications",
        },
    ]
}

pub(in crate::tui_shell) fn superpositions_command_defs() -> Vec<CommandDef> {
    vec![
        CommandDef {
            name: "back",
            aliases: &[],
            usage: "back",
            help: "Return to root",
        },
        CommandDef {
            name: "pick",
            aliases: &[],
            usage: "pick <n>",
            help: "Pick variant for selected path",
        },
        CommandDef {
            name: "clear",
            aliases: &[],
            usage: "clear",
            help: "Clear decision for selected path",
        },
        CommandDef {
            name: "next-missing",
            aliases: &[],
            usage: "next-missing",
            help: "Jump to next missing decision",
        },
        CommandDef {
            name: "next-invalid",
            aliases: &[],
            usage: "next-invalid",
            help: "Jump to next invalid decision",
        },
        CommandDef {
            name: "validate",
            aliases: &[],
            usage: "validate",
            help: "Recompute validation",
        },
        CommandDef {
            name: "apply",
            aliases: &[],
            usage: "apply [publish]",
            help: "Apply resolution and optionally publish",
        },
    ]
}
