use crate::tui_shell::{CommandDef, RootContext};

pub(in crate::tui_shell) fn global_command_defs() -> Vec<CommandDef> {
    vec![
        CommandDef {
            name: "help",
            aliases: &["h", "?"],
            usage: "help [command]",
            help: "Show help",
        },
        CommandDef {
            name: "settings",
            aliases: &[],
            usage: "settings",
            help: "Open settings",
        },
        CommandDef {
            name: "quit",
            aliases: &[],
            usage: "quit",
            help: "Exit",
        },
    ]
}

fn auth_command_defs() -> Vec<CommandDef> {
    vec![
        CommandDef {
            name: "login",
            aliases: &[],
            usage: "login",
            help: "Login (guided prompt)",
        },
        CommandDef {
            name: "logout",
            aliases: &[],
            usage: "logout",
            help: "Clear stored remote token",
        },
    ]
}

pub(in crate::tui_shell) fn local_root_command_defs() -> Vec<CommandDef> {
    let mut out = global_command_defs();
    out.extend(auth_command_defs());
    out.extend(vec![
        CommandDef {
            name: "status",
            aliases: &["st"],
            usage: "status",
            help: "Refresh local status root view",
        },
        CommandDef {
            name: "refresh",
            aliases: &["r"],
            usage: "refresh",
            help: "Refresh local status root view",
        },
        CommandDef {
            name: "init",
            aliases: &[],
            usage: "init [force]",
            help: "Initialize a workspace (.converge)",
        },
        CommandDef {
            name: "snap",
            aliases: &["save"],
            usage: "snap [message...]",
            help: "Create a snapshot",
        },
        CommandDef {
            name: "publish",
            aliases: &[],
            usage: "publish [edit]",
            help: "Publish a snap to remote",
        },
        CommandDef {
            name: "sync",
            aliases: &[],
            usage: "sync [edit]",
            help: "Sync to your lane (guided prompt)",
        },
        CommandDef {
            name: "history",
            aliases: &[],
            usage: "history [N]",
            help: "Browse saved snapshots",
        },
        CommandDef {
            name: "show",
            aliases: &[],
            usage: "show <snap_id>",
            help: "Show a snap",
        },
        CommandDef {
            name: "restore",
            aliases: &[],
            usage: "restore <snap> [force]",
            help: "Restore a snap into the working directory",
        },
        CommandDef {
            name: "move",
            aliases: &["mv"],
            usage: "move [<from>] [<to>]",
            help: "Move/rename a path (guided; case-safe)",
        },
        CommandDef {
            name: "purge",
            aliases: &[],
            usage: "purge [dry]",
            help: "Purge local objects (per retention policy)",
        },
        CommandDef {
            name: "clear",
            aliases: &[],
            usage: "clear",
            help: "Clear last output/log",
        },
    ]);
    out
}

pub(in crate::tui_shell) fn remote_root_command_defs() -> Vec<CommandDef> {
    let mut out = global_command_defs();
    out.extend(auth_command_defs());
    out.extend(vec![
        CommandDef {
            name: "bootstrap",
            aliases: &[],
            usage: "bootstrap",
            help: "Bootstrap first admin (guided)",
        },
        CommandDef {
            name: "create-repo",
            aliases: &[],
            usage: "create-repo",
            help: "Create the configured repo on the server",
        },
        CommandDef {
            name: "gates",
            aliases: &["gate-graph"],
            usage: "gates",
            help: "View gate graph (admin)",
        },
        CommandDef {
            name: "status",
            aliases: &["st"],
            usage: "status",
            help: "Show detailed status (modal)",
        },
        CommandDef {
            name: "refresh",
            aliases: &["r"],
            usage: "refresh",
            help: "Refresh dashboard",
        },
        CommandDef {
            name: "remote",
            aliases: &[],
            usage: "remote show|ping|set|unset",
            help: "Show/ping the configured remote",
        },
        CommandDef {
            name: "ping",
            aliases: &[],
            usage: "ping",
            help: "Ping remote /healthz",
        },
        CommandDef {
            name: "fetch",
            aliases: &[],
            usage: "fetch",
            help: "Fetch publications or lane heads into local store",
        },
        CommandDef {
            name: "lanes",
            aliases: &[],
            usage: "lanes",
            help: "List lanes and lane heads",
        },
        CommandDef {
            name: "releases",
            aliases: &[],
            usage: "releases",
            help: "Open releases browser",
        },
        CommandDef {
            name: "members",
            aliases: &[],
            usage: "members",
            help: "Show repo and lane membership",
        },
        CommandDef {
            name: "member",
            aliases: &[],
            usage: "member",
            help: "Manage repo membership (guided prompt)",
        },
        CommandDef {
            name: "lane-member",
            aliases: &[],
            usage: "lane-member",
            help: "Manage lane membership (guided prompt)",
        },
        CommandDef {
            name: "inbox",
            aliases: &[],
            usage: "inbox [edit]",
            help: "Open inbox browser",
        },
        CommandDef {
            name: "bundles",
            aliases: &[],
            usage: "bundles [edit]",
            help: "Open bundles browser",
        },
        CommandDef {
            name: "bundle",
            aliases: &[],
            usage: "bundle",
            help: "Create a bundle (opens Inbox)",
        },
        CommandDef {
            name: "pins",
            aliases: &[],
            usage: "pins",
            help: "List pinned bundles",
        },
        CommandDef {
            name: "pin",
            aliases: &[],
            usage: "pin",
            help: "Pin/unpin a bundle (guided)",
        },
        CommandDef {
            name: "approve",
            aliases: &[],
            usage: "approve",
            help: "Approve a bundle (guided)",
        },
        CommandDef {
            name: "promote",
            aliases: &[],
            usage: "promote",
            help: "Promote a bundle (guided)",
        },
        CommandDef {
            name: "release",
            aliases: &[],
            usage: "release",
            help: "Create a release (guided)",
        },
        CommandDef {
            name: "superpositions",
            aliases: &["supers"],
            usage: "superpositions",
            help: "Open superpositions (guided)",
        },
    ]);
    out
}

pub(in crate::tui_shell) fn root_command_defs(ctx: RootContext) -> Vec<CommandDef> {
    match ctx {
        RootContext::Local => local_root_command_defs(),
        RootContext::Remote => remote_root_command_defs(),
    }
}
