use std::io::{self, IsTerminal};
use std::time::Duration;

use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use crate::model::{ObjectId, RemoteConfig, Resolution, ResolutionDecision};
use crate::remote::RemoteClient;
use crate::resolve::{ResolutionValidation, superposition_variants, validate_resolution};
use crate::workspace::Workspace;

pub fn run() -> Result<()> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        anyhow::bail!("TUI requires an interactive terminal (TTY)");
    }

    let mut stdout = io::stdout();
    enable_raw_mode().context("enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("enter alternate screen")?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("create terminal")?;
    terminal.clear().ok();

    let mut app = App::load();
    let res = run_loop(&mut terminal, &mut app);

    disable_raw_mode().ok();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).ok();
    terminal.show_cursor().ok();

    res
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Mode {
    #[default]
    Local,
    Remote,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EntryKind {
    Command,
    Output,
    Error,
}

#[derive(Clone, Debug)]
struct ScrollEntry {
    ts: String,
    kind: EntryKind,
    lines: Vec<String>,
}

#[derive(Debug)]
enum Panel {
    Status {
        title: String,
        updated_at: String,
        lines: Vec<String>,
    },
    Inbox {
        title: String,
        updated_at: String,
        scope: String,
        gate: String,
        filter: Option<String>,
        items: Vec<crate::remote::Publication>,
        selected: usize,
    },
    Bundles {
        title: String,
        updated_at: String,
        scope: String,
        gate: String,
        filter: Option<String>,
        items: Vec<crate::remote::Bundle>,
        selected: usize,
    },
    Superpositions {
        title: String,
        updated_at: String,
        bundle_id: String,
        filter: Option<String>,
        root_manifest: ObjectId,
        variants: std::collections::BTreeMap<String, Vec<crate::model::SuperpositionVariant>>,
        decisions: std::collections::BTreeMap<String, ResolutionDecision>,
        validation: Option<ResolutionValidation>,
        items: Vec<(String, usize)>,
        selected: usize,
    },
}

impl Panel {
    fn title(&self) -> &str {
        match self {
            Panel::Status { title, .. } => title,
            Panel::Inbox { title, .. } => title,
            Panel::Bundles { title, .. } => title,
            Panel::Superpositions { title, .. } => title,
        }
    }

    fn updated_at(&self) -> &str {
        match self {
            Panel::Status { updated_at, .. } => updated_at,
            Panel::Inbox { updated_at, .. } => updated_at,
            Panel::Bundles { updated_at, .. } => updated_at,
            Panel::Superpositions { updated_at, .. } => updated_at,
        }
    }

    fn selected(&self) -> Option<usize> {
        match self {
            Panel::Status { .. } => None,
            Panel::Inbox {
                selected, items, ..
            } => {
                if items.is_empty() {
                    None
                } else {
                    Some((*selected).min(items.len().saturating_sub(1)))
                }
            }
            Panel::Bundles {
                selected, items, ..
            } => {
                if items.is_empty() {
                    None
                } else {
                    Some((*selected).min(items.len().saturating_sub(1)))
                }
            }
            Panel::Superpositions {
                selected, items, ..
            } => {
                if items.is_empty() {
                    None
                } else {
                    Some((*selected).min(items.len().saturating_sub(1)))
                }
            }
        }
    }

    fn move_up(&mut self) {
        match self {
            Panel::Inbox { selected, .. }
            | Panel::Bundles { selected, .. }
            | Panel::Superpositions { selected, .. } => {
                *selected = selected.saturating_sub(1);
            }
            Panel::Status { .. } => {}
        }
    }

    fn move_down(&mut self) {
        match self {
            Panel::Inbox { selected, .. }
            | Panel::Bundles { selected, .. }
            | Panel::Superpositions { selected, .. } => {
                *selected = selected.saturating_add(1);
            }
            Panel::Status { .. } => {}
        }
    }
}

#[derive(Default)]
struct Input {
    buf: String,
    cursor: usize,
    history: Vec<String>,
    history_pos: Option<usize>,
}

impl Input {
    fn clear(&mut self) {
        self.buf.clear();
        self.cursor = 0;
        self.history_pos = None;
    }

    fn insert_char(&mut self, c: char) {
        self.buf.insert(self.cursor, c);
        self.cursor += 1;
    }

    fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
        self.buf.remove(self.cursor);
    }

    fn delete(&mut self) {
        if self.cursor >= self.buf.len() {
            return;
        }
        self.buf.remove(self.cursor);
    }

    fn move_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    fn move_right(&mut self) {
        self.cursor = (self.cursor + 1).min(self.buf.len());
    }

    fn set(&mut self, s: String) {
        self.buf = s;
        self.cursor = self.buf.len();
    }

    fn push_history(&mut self, line: &str) {
        let line = line.trim();
        if line.is_empty() {
            return;
        }
        if self.history.last().map(|s| s.as_str()) == Some(line) {
            return;
        }
        self.history.push(line.to_string());
        self.history_pos = None;
    }

    fn history_up(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let next = match self.history_pos {
            None => self.history.len().saturating_sub(1),
            Some(i) => i.saturating_sub(1),
        };
        self.history_pos = Some(next);
        self.set(self.history[next].clone());
    }

    fn history_down(&mut self) {
        let Some(i) = self.history_pos else {
            return;
        };
        if i + 1 >= self.history.len() {
            self.history_pos = None;
            self.clear();
            return;
        }
        let next = i + 1;
        self.history_pos = Some(next);
        self.set(self.history[next].clone());
    }
}

#[derive(Clone, Debug)]
struct CommandDef {
    name: &'static str,
    aliases: &'static [&'static str],
    usage: &'static str,
    help: &'static str,
}

fn command_defs() -> Vec<CommandDef> {
    vec![
        CommandDef {
            name: "help",
            aliases: &["h", "?"],
            usage: "help [command]",
            help: "Show help",
        },
        CommandDef {
            name: "status",
            aliases: &["st"],
            usage: "status",
            help: "Show workspace status",
        },
        CommandDef {
            name: "init",
            aliases: &[],
            usage: "init [--force]",
            help: "Initialize a workspace (.converge)",
        },
        CommandDef {
            name: "snap",
            aliases: &[],
            usage: "snap [-m <message>]",
            help: "Create a snap",
        },
        CommandDef {
            name: "snaps",
            aliases: &[],
            usage: "snaps [--limit N]",
            help: "List snaps",
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
            usage: "restore <snap_id> [--force]",
            help: "Restore a snap into the working directory",
        },
        CommandDef {
            name: "clear",
            aliases: &[],
            usage: "clear",
            help: "Clear scrollback",
        },
        CommandDef {
            name: "panel",
            aliases: &[],
            usage: "panel show|close",
            help: "Show or close the side panel",
        },
        CommandDef {
            name: "quit",
            aliases: &["q", "exit"],
            usage: "quit",
            help: "Quit",
        },
        // Remote-mode commands (Phase 013)
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
            name: "publish",
            aliases: &[],
            usage: "publish [--snap-id <id>] [--scope <id>] [--gate <id>]",
            help: "Publish a snap to remote",
        },
        CommandDef {
            name: "fetch",
            aliases: &[],
            usage: "fetch [--snap-id <id>]",
            help: "Fetch remote publications into local store",
        },
        CommandDef {
            name: "inbox",
            aliases: &[],
            usage: "inbox [--scope <id>] [--gate <id>] [--filter <q>] [--limit N]",
            help: "List remote publications",
        },
        CommandDef {
            name: "bundles",
            aliases: &[],
            usage: "bundles [--scope <id>] [--gate <id>] [--filter <q>] [--limit N]",
            help: "List remote bundles",
        },
        CommandDef {
            name: "bundle",
            aliases: &[],
            usage: "bundle [--scope <id>] [--gate <id>] [--publication <id>...]",
            help: "Create a bundle from publications",
        },
        CommandDef {
            name: "approve",
            aliases: &[],
            usage: "approve --bundle-id <id>",
            help: "Approve a bundle",
        },
        CommandDef {
            name: "promote",
            aliases: &[],
            usage: "promote --bundle-id <id> [--to-gate <id>]",
            help: "Promote a bundle",
        },
        CommandDef {
            name: "superpositions",
            aliases: &["supers"],
            usage: "superpositions --bundle-id <id> [--filter <q>]",
            help: "List conflicted paths in a bundle",
        },
    ]
}

fn now_ts() -> String {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "<time>".to_string())
}

#[derive(Default)]
struct App {
    mode: Mode,
    workspace: Option<Workspace>,
    workspace_err: Option<String>,

    scroll: Vec<ScrollEntry>,
    input: Input,

    suggestions: Vec<CommandDef>,
    suggestion_selected: usize,

    panel: Option<Panel>,

    quit: bool,
}

impl App {
    fn load() -> Self {
        let mut app = App::default();
        let cwd = match std::env::current_dir() {
            Ok(p) => p,
            Err(err) => {
                app.workspace_err = Some(format!("get current dir: {:#}", err));
                return app;
            }
        };

        match Workspace::discover(&cwd) {
            Ok(ws) => {
                app.workspace = Some(ws);
            }
            Err(err) => {
                app.workspace_err = Some(format!("{}", err));
            }
        }

        app.push_output(vec![
            "Type `help` for commands.".to_string(),
            "(Leading `/` is optional; it forces command mode.)".to_string(),
        ]);
        app
    }

    fn prompt(&self) -> &'static str {
        match self.mode {
            Mode::Local => "local>",
            Mode::Remote => "remote>",
        }
    }

    fn push_entry(&mut self, kind: EntryKind, lines: Vec<String>) {
        self.scroll.push(ScrollEntry {
            ts: now_ts(),
            kind,
            lines,
        });
    }

    fn push_command(&mut self, line: String) {
        self.push_entry(EntryKind::Command, vec![line]);
    }

    fn push_output(&mut self, lines: Vec<String>) {
        self.push_entry(EntryKind::Output, lines);
    }

    fn push_error(&mut self, msg: String) {
        self.push_entry(EntryKind::Error, vec![msg]);
    }

    fn set_panel(&mut self, title: &str, lines: Vec<String>) {
        self.panel = Some(Panel::Status {
            title: title.to_string(),
            updated_at: now_ts(),
            lines,
        });
    }

    fn clear_panel(&mut self) {
        self.panel = None;
    }

    fn recompute_suggestions(&mut self) {
        let q = self.input.buf.trim_start_matches('/').trim().to_lowercase();
        if q.is_empty() {
            self.suggestions.clear();
            self.suggestion_selected = 0;
            return;
        }

        // Only match the first token for palette.
        let first = q.split_whitespace().next().unwrap_or("");
        if first.is_empty() {
            self.suggestions.clear();
            self.suggestion_selected = 0;
            return;
        }

        let mut defs = command_defs();
        defs.sort_by(|a, b| a.name.cmp(b.name));

        let mut scored = Vec::new();
        for d in defs {
            let mut best = score_match(first, d.name);
            for &a in d.aliases {
                best = best.max(score_match(first, a));
            }
            if best > 0 {
                scored.push((best, d));
            }
        }

        scored.sort_by(|(sa, a), (sb, b)| sb.cmp(sa).then_with(|| a.name.cmp(b.name)));
        self.suggestions = scored.into_iter().map(|(_, d)| d).take(5).collect();
        self.suggestion_selected = self.suggestion_selected.min(self.suggestions.len());
    }

    fn apply_selected_suggestion(&mut self) {
        if self.suggestions.is_empty() {
            return;
        }
        let sel = self
            .suggestion_selected
            .min(self.suggestions.len().saturating_sub(1));
        let cmd = self.suggestions[sel].name;

        let raw = self.input.buf.trim_start_matches('/');
        let trimmed = raw.trim_start();
        let mut iter = trimmed.splitn(2, char::is_whitespace);
        let first = iter.next().unwrap_or("");
        let rest = iter.next().unwrap_or("");

        if first.is_empty() {
            self.input.set(format!("{} ", cmd));
        } else {
            // Replace first token.
            if rest.is_empty() {
                self.input.set(format!("{} ", cmd));
            } else {
                self.input.set(format!("{} {}", cmd, rest.trim_start()));
            }
        }
        self.recompute_suggestions();
    }

    fn run_current_input(&mut self) {
        let line = self.input.buf.trim().to_string();
        if line.is_empty() {
            return;
        }

        self.input.push_history(&line);
        self.push_command(format!("{} {}", self.prompt(), line));
        self.input.clear();
        self.suggestions.clear();
        self.suggestion_selected = 0;

        let line = line.strip_prefix('/').unwrap_or(&line).trim();
        let tokens = match tokenize(line) {
            Ok(t) => t,
            Err(err) => {
                self.push_error(format!("parse error: {}", err));
                return;
            }
        };
        if tokens.is_empty() {
            return;
        }

        let mut cmd = tokens[0].to_lowercase();
        let args = &tokens[1..];

        // Resolve aliases.
        let mut defs = command_defs();
        defs.sort_by(|a, b| a.name.cmp(b.name));
        if let Some(d) = defs.iter().find(|d| d.name == cmd) {
            let _ = d;
        } else if let Some(d) = defs.iter().find(|d| d.aliases.iter().any(|&a| a == cmd)) {
            cmd = d.name.to_string();
        } else {
            // Try prefix match if unambiguous.
            let matches = defs
                .iter()
                .filter(|d| d.name.starts_with(&cmd))
                .collect::<Vec<_>>();
            if matches.len() == 1 {
                cmd = matches[0].name.to_string();
            }
        }

        match cmd.as_str() {
            "help" => self.cmd_help(args),
            "status" => self.cmd_status(args),
            "init" => self.cmd_init(args),
            "snap" => self.cmd_snap(args),
            "snaps" => self.cmd_snaps(args),
            "show" => self.cmd_show(args),
            "restore" => self.cmd_restore(args),

            "panel" => self.cmd_panel(args),

            "remote" => self.cmd_remote(args),
            "ping" => self.cmd_ping(args),
            "publish" => self.cmd_publish(args),
            "fetch" => self.cmd_fetch(args),
            "inbox" => self.cmd_inbox(args),
            "bundles" => self.cmd_bundles(args),
            "bundle" => self.cmd_bundle(args),
            "approve" => self.cmd_approve(args),
            "promote" => self.cmd_promote(args),
            "superpositions" => self.cmd_superpositions(args),
            "supers" => self.cmd_superpositions(args),

            "clear" => {
                self.scroll.clear();
            }
            "quit" => {
                self.quit = true;
            }
            _ => {
                self.push_error(format!("unknown command: {}", cmd));
            }
        }
    }

    fn require_workspace(&mut self) -> Option<Workspace> {
        match self.workspace.clone() {
            Some(ws) => Some(ws),
            None => {
                let msg = self
                    .workspace_err
                    .clone()
                    .unwrap_or_else(|| "not in a converge workspace".to_string());
                self.push_error(msg);
                None
            }
        }
    }

    fn cmd_help(&mut self, args: &[String]) {
        let defs = command_defs();
        if args.is_empty() {
            let mut lines = Vec::new();
            lines.push("Commands:".to_string());
            let mut defs = defs;
            defs.sort_by(|a, b| a.name.cmp(b.name));
            for d in defs {
                lines.push(format!("- {:<10} {}", d.name, d.help));
            }
            lines.push("".to_string());
            lines.push("Notes:".to_string());
            lines.push("- Leading `/` is optional; it forces command mode.".to_string());
            lines.push("- Tab toggles Local/Remote mode only when input is empty.".to_string());
            self.push_output(lines);
            return;
        }

        let q = args[0].to_lowercase();
        let Some(d) = defs
            .iter()
            .find(|d| d.name == q || d.aliases.iter().any(|&a| a == q))
        else {
            self.push_error(format!("unknown command: {}", q));
            return;
        };

        self.push_output(vec![
            format!("{} - {}", d.name, d.help),
            format!("usage: {}", d.usage),
        ]);
    }

    fn remote_config(&mut self) -> Option<RemoteConfig> {
        let ws = self.require_workspace()?;
        let cfg = match ws.store.read_config() {
            Ok(c) => c,
            Err(err) => {
                self.push_error(format!("read config: {:#}", err));
                return None;
            }
        };
        cfg.remote
    }

    fn remote_client(&mut self) -> Option<RemoteClient> {
        let cfg = self.remote_config()?;
        match RemoteClient::new(cfg) {
            Ok(c) => Some(c),
            Err(err) => {
                self.push_error(format!("init remote client: {:#}", err));
                None
            }
        }
    }

    fn cmd_status(&mut self, _args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };
        let cfg = match ws.store.read_config() {
            Ok(c) => c,
            Err(err) => {
                self.push_error(format!("read config: {:#}", err));
                return;
            }
        };

        let snaps = match ws.list_snaps() {
            Ok(s) => s,
            Err(err) => {
                self.push_error(format!("list snaps: {:#}", err));
                return;
            }
        };

        let mut lines = Vec::new();
        lines.push(format!("mode: {:?}", self.mode));
        lines.push(format!("workspace: {}", ws.root.display()));
        lines.push(format!(
            "remote: {}",
            if cfg.remote.is_some() {
                "configured"
            } else {
                "not configured"
            }
        ));
        lines.push(format!("snaps: {}", snaps.len()));
        if let Some(s) = snaps.first() {
            lines.push(format!("latest: {} {}", s.id, s.created_at));
        }
        self.push_output(lines.clone());
        self.set_panel("Status", lines);
    }

    fn cmd_init(&mut self, args: &[String]) {
        let mut force = false;
        for a in args {
            match a.as_str() {
                "--force" => force = true,
                _ => {
                    self.push_error(format!("unknown flag: {}", a));
                    return;
                }
            }
        }

        let cwd = match std::env::current_dir() {
            Ok(p) => p,
            Err(err) => {
                self.push_error(format!("get current dir: {:#}", err));
                return;
            }
        };

        match Workspace::init(&cwd, force) {
            Ok(ws) => {
                self.workspace = Some(ws);
                self.workspace_err = None;
                self.push_output(vec!["initialized .converge".to_string()]);
            }
            Err(err) => {
                self.push_error(format!("init: {:#}", err));
            }
        }
    }

    fn cmd_snap(&mut self, args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };

        let mut message: Option<String> = None;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-m" | "--message" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for -m/--message".to_string());
                        return;
                    }
                    message = Some(args[i].clone());
                }
                a => {
                    self.push_error(format!("unknown arg: {}", a));
                    return;
                }
            }
            i += 1;
        }

        match ws.create_snap(message) {
            Ok(snap) => {
                self.push_output(vec![format!("snap {}", snap.id)]);
            }
            Err(err) => {
                self.push_error(format!("snap: {:#}", err));
            }
        }
    }

    fn cmd_snaps(&mut self, args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };

        let mut limit: Option<usize> = None;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--limit" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --limit".to_string());
                        return;
                    }
                    limit = match args[i].parse::<usize>() {
                        Ok(n) => Some(n),
                        Err(_) => {
                            self.push_error("invalid --limit".to_string());
                            return;
                        }
                    };
                }
                a => {
                    self.push_error(format!("unknown arg: {}", a));
                    return;
                }
            }
            i += 1;
        }

        match ws.list_snaps() {
            Ok(snaps) => {
                let mut lines = Vec::new();
                let snaps = if let Some(n) = limit {
                    snaps.into_iter().take(n).collect::<Vec<_>>()
                } else {
                    snaps
                };
                for s in snaps {
                    let short = s.id.chars().take(8).collect::<String>();
                    let msg = s.message.unwrap_or_default();
                    if msg.is_empty() {
                        lines.push(format!("{} {}", short, s.created_at));
                    } else {
                        lines.push(format!("{} {} {}", short, s.created_at, msg));
                    }
                }
                if lines.is_empty() {
                    lines.push("(no snaps)".to_string());
                }
                self.push_output(lines);
            }
            Err(err) => {
                self.push_error(format!("snaps: {:#}", err));
            }
        }
    }

    fn cmd_show(&mut self, args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };
        if args.len() != 1 {
            self.push_error("usage: show <snap_id>".to_string());
            return;
        }
        match ws.show_snap(&args[0]) {
            Ok(s) => {
                let mut lines = Vec::new();
                lines.push(format!("id: {}", s.id));
                lines.push(format!("created_at: {}", s.created_at));
                if let Some(msg) = s.message
                    && !msg.is_empty()
                {
                    lines.push(format!("message: {}", msg));
                }
                lines.push(format!("root_manifest: {}", s.root_manifest.as_str()));
                lines.push(format!(
                    "stats: files={} dirs={} symlinks={} bytes={}",
                    s.stats.files, s.stats.dirs, s.stats.symlinks, s.stats.bytes
                ));
                self.push_output(lines);
            }
            Err(err) => {
                self.push_error(format!("show: {:#}", err));
            }
        }
    }

    fn cmd_restore(&mut self, args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };
        if args.is_empty() {
            self.push_error("usage: restore <snap_id> [--force]".to_string());
            return;
        }

        let mut snap_id = None;
        let mut force = false;
        for a in args {
            if a == "--force" {
                force = true;
                continue;
            }
            if snap_id.is_none() {
                snap_id = Some(a.clone());
                continue;
            }
            self.push_error(format!("unknown arg: {}", a));
            return;
        }

        let Some(snap_id) = snap_id else {
            self.push_error("missing snap_id".to_string());
            return;
        };

        match ws.restore_snap(&snap_id, force) {
            Ok(()) => self.push_output(vec![format!("restored {}", snap_id)]),
            Err(err) => self.push_error(format!("restore: {:#}", err)),
        }
    }

    fn cmd_panel(&mut self, args: &[String]) {
        let sub = args.first().map(|s| s.as_str()).unwrap_or("show");
        match sub {
            "show" => {
                if let Some(p) = &self.panel {
                    self.push_output(vec![
                        format!("panel: {}", p.title()),
                        format!("updated_at: {}", p.updated_at()),
                    ]);
                } else {
                    self.push_output(vec!["(no panel)".to_string()]);
                }
            }
            "close" => {
                self.clear_panel();
                self.push_output(vec!["panel closed".to_string()]);
            }
            _ => {
                self.push_error("usage: panel show|close".to_string());
            }
        }
    }

    fn cmd_remote(&mut self, args: &[String]) {
        let sub = args.first().map(|s| s.as_str()).unwrap_or("show");
        match sub {
            "show" => {
                let Some(cfg) = self.remote_config() else {
                    self.push_error("no remote configured".to_string());
                    return;
                };
                self.push_output(vec![
                    format!("url: {}", cfg.base_url),
                    format!("repo: {}", cfg.repo_id),
                    format!("scope: {}", cfg.scope),
                    format!("gate: {}", cfg.gate),
                ]);
            }
            "ping" => {
                self.cmd_ping(&[]);
            }
            "set" => {
                self.cmd_remote_set(&args[1..]);
            }
            "unset" => {
                self.cmd_remote_unset(&args[1..]);
            }
            _ => {
                self.push_error("usage: remote show|ping|set|unset".to_string());
            }
        }
    }

    fn cmd_remote_set(&mut self, args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };

        let mut url: Option<String> = None;
        let mut token: Option<String> = None;
        let mut repo: Option<String> = None;
        let mut scope: Option<String> = None;
        let mut gate: Option<String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--url" => {
                    i += 1;
                    url = args.get(i).cloned();
                }
                "--token" => {
                    i += 1;
                    token = args.get(i).cloned();
                }
                "--repo" => {
                    i += 1;
                    repo = args.get(i).cloned();
                }
                "--scope" => {
                    i += 1;
                    scope = args.get(i).cloned();
                }
                "--gate" => {
                    i += 1;
                    gate = args.get(i).cloned();
                }
                a => {
                    self.push_error(format!("unknown arg: {}", a));
                    return;
                }
            }
            if i >= args.len() {
                self.push_error("missing value for flag".to_string());
                return;
            }
            i += 1;
        }

        let (Some(base_url), Some(token), Some(repo_id), Some(scope), Some(gate)) =
            (url, token, repo, scope, gate)
        else {
            self.push_error(
                "usage: remote set --url <url> --token <token> --repo <id> --scope <id> --gate <id>"
                    .to_string(),
            );
            return;
        };

        let mut cfg = match ws.store.read_config() {
            Ok(c) => c,
            Err(err) => {
                self.push_error(format!("read config: {:#}", err));
                return;
            }
        };

        cfg.remote = Some(RemoteConfig {
            base_url,
            token,
            repo_id,
            scope,
            gate,
        });

        if let Err(err) = ws.store.write_config(&cfg) {
            self.push_error(format!("write config: {:#}", err));
            return;
        }

        self.push_output(vec!["remote configured".to_string()]);
    }

    fn cmd_remote_unset(&mut self, args: &[String]) {
        let _ = args;
        let Some(ws) = self.require_workspace() else {
            return;
        };

        let mut cfg = match ws.store.read_config() {
            Ok(c) => c,
            Err(err) => {
                self.push_error(format!("read config: {:#}", err));
                return;
            }
        };

        cfg.remote = None;
        if let Err(err) = ws.store.write_config(&cfg) {
            self.push_error(format!("write config: {:#}", err));
            return;
        }
        self.push_output(vec!["remote unset".to_string()]);
    }

    fn cmd_ping(&mut self, _args: &[String]) {
        let Some(cfg) = self.remote_config() else {
            self.push_error("no remote configured".to_string());
            return;
        };

        let url = format!("{}/healthz", cfg.base_url.trim_end_matches('/'));
        let start = std::time::Instant::now();
        let resp = reqwest::blocking::get(&url);
        match resp {
            Ok(r) => {
                let ms = start.elapsed().as_millis();
                self.push_output(vec![format!("{} {}ms", r.status(), ms)]);
            }
            Err(err) => {
                self.push_error(format!("ping failed: {:#}", err));
            }
        }
    }

    fn cmd_publish(&mut self, args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };
        let Some(cfg) = self.remote_config() else {
            self.push_error("no remote configured".to_string());
            return;
        };

        let mut snap_id: Option<String> = None;
        let mut scope: Option<String> = None;
        let mut gate: Option<String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--snap-id" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --snap-id".to_string());
                        return;
                    }
                    snap_id = Some(args[i].clone());
                }
                "--scope" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --scope".to_string());
                        return;
                    }
                    scope = Some(args[i].clone());
                }
                "--gate" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --gate".to_string());
                        return;
                    }
                    gate = Some(args[i].clone());
                }
                a => {
                    self.push_error(format!("unknown arg: {}", a));
                    return;
                }
            }
            i += 1;
        }

        let snap_id = match snap_id {
            Some(id) => id,
            None => match ws.list_snaps() {
                Ok(snaps) => match snaps.first() {
                    Some(s) => s.id.clone(),
                    None => {
                        self.push_error("no snaps to publish".to_string());
                        return;
                    }
                },
                Err(err) => {
                    self.push_error(format!("list snaps: {:#}", err));
                    return;
                }
            },
        };

        let snap = match ws.store.get_snap(&snap_id) {
            Ok(s) => s,
            Err(err) => {
                self.push_error(format!("read snap: {:#}", err));
                return;
            }
        };

        let client = match RemoteClient::new(cfg.clone()) {
            Ok(c) => c,
            Err(err) => {
                self.push_error(format!("init remote client: {:#}", err));
                return;
            }
        };

        let scope = scope.unwrap_or(cfg.scope);
        let gate = gate.unwrap_or(cfg.gate);

        match client.publish_snap(&ws.store, &snap, &scope, &gate) {
            Ok(p) => {
                self.push_output(vec![format!("published {}", p.id)]);
            }
            Err(err) => {
                self.push_error(format!("publish: {:#}", err));
            }
        }
    }

    fn cmd_fetch(&mut self, args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };
        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };

        let mut snap_id: Option<String> = None;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--snap-id" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --snap-id".to_string());
                        return;
                    }
                    snap_id = Some(args[i].clone());
                }
                a => {
                    self.push_error(format!("unknown arg: {}", a));
                    return;
                }
            }
            i += 1;
        }

        match client.fetch_publications(&ws.store, snap_id.as_deref()) {
            Ok(fetched) => {
                self.push_output(vec![format!("fetched {} snaps", fetched.len())]);
            }
            Err(err) => {
                self.push_error(format!("fetch: {:#}", err));
            }
        }
    }

    fn cmd_inbox(&mut self, args: &[String]) {
        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };
        let cfg = match self.remote_config() {
            Some(c) => c,
            None => return,
        };

        let mut scope: Option<String> = None;
        let mut gate: Option<String> = None;
        let mut limit: Option<usize> = None;
        let mut filter: Option<String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--scope" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --scope".to_string());
                        return;
                    }
                    scope = Some(args[i].clone());
                }
                "--gate" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --gate".to_string());
                        return;
                    }
                    gate = Some(args[i].clone());
                }
                "--limit" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --limit".to_string());
                        return;
                    }
                    limit = match args[i].parse::<usize>() {
                        Ok(n) => Some(n),
                        Err(_) => {
                            self.push_error("invalid --limit".to_string());
                            return;
                        }
                    };
                }
                "--filter" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --filter".to_string());
                        return;
                    }
                    filter = Some(args[i].clone());
                }
                a => {
                    self.push_error(format!("unknown arg: {}", a));
                    return;
                }
            }
            i += 1;
        }

        let scope = scope.unwrap_or(cfg.scope);
        let gate = gate.unwrap_or(cfg.gate);
        let filter_lc = filter.as_ref().map(|s| s.to_lowercase());

        let pubs = match client.list_publications() {
            Ok(p) => p,
            Err(err) => {
                self.push_error(format!("inbox: {:#}", err));
                return;
            }
        };

        let mut pubs = pubs
            .into_iter()
            .filter(|p| p.scope == scope && p.gate == gate)
            .filter(|p| {
                let Some(q) = filter_lc.as_deref() else {
                    return true;
                };
                if p.id.to_lowercase().contains(q)
                    || p.snap_id.to_lowercase().contains(q)
                    || p.publisher.to_lowercase().contains(q)
                    || p.created_at.to_lowercase().contains(q)
                {
                    return true;
                }
                if let Some(r) = &p.resolution
                    && r.bundle_id.to_lowercase().contains(q)
                {
                    return true;
                }
                false
            })
            .collect::<Vec<_>>();
        pubs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        if let Some(n) = limit {
            pubs.truncate(n);
        }

        if pubs.is_empty() {
            let lines = vec!["(empty)".to_string()];
            self.push_output(lines.clone());
            self.panel = Some(Panel::Inbox {
                title: "Inbox".to_string(),
                updated_at: now_ts(),
                scope,
                gate,
                filter,
                items: Vec::new(),
                selected: 0,
            });
            return;
        }

        let mut lines = Vec::new();
        for p in &pubs {
            let sid = p.snap_id.chars().take(8).collect::<String>();
            let rid = p.id.chars().take(8).collect::<String>();
            let res = if p.resolution.is_some() {
                " resolved"
            } else {
                ""
            };
            lines.push(format!("{} {} {}{}", rid, sid, p.created_at, res));
        }
        self.push_output(lines);
        self.panel = Some(Panel::Inbox {
            title: "Inbox".to_string(),
            updated_at: now_ts(),
            scope,
            gate,
            filter,
            items: pubs,
            selected: 0,
        });
    }

    fn cmd_bundles(&mut self, args: &[String]) {
        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };
        let cfg = match self.remote_config() {
            Some(c) => c,
            None => return,
        };

        let mut scope: Option<String> = None;
        let mut gate: Option<String> = None;
        let mut limit: Option<usize> = None;
        let mut filter: Option<String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--scope" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --scope".to_string());
                        return;
                    }
                    scope = Some(args[i].clone());
                }
                "--gate" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --gate".to_string());
                        return;
                    }
                    gate = Some(args[i].clone());
                }
                "--limit" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --limit".to_string());
                        return;
                    }
                    limit = match args[i].parse::<usize>() {
                        Ok(n) => Some(n),
                        Err(_) => {
                            self.push_error("invalid --limit".to_string());
                            return;
                        }
                    };
                }
                "--filter" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --filter".to_string());
                        return;
                    }
                    filter = Some(args[i].clone());
                }
                a => {
                    self.push_error(format!("unknown arg: {}", a));
                    return;
                }
            }
            i += 1;
        }

        let scope = scope.unwrap_or(cfg.scope);
        let gate = gate.unwrap_or(cfg.gate);
        let filter_lc = filter.as_ref().map(|s| s.to_lowercase());

        let bundles = match client.list_bundles() {
            Ok(b) => b,
            Err(err) => {
                self.push_error(format!("bundles: {:#}", err));
                return;
            }
        };

        let mut bundles = bundles
            .into_iter()
            .filter(|b| b.scope == scope && b.gate == gate)
            .filter(|b| {
                let Some(q) = filter_lc.as_deref() else {
                    return true;
                };
                if b.id.to_lowercase().contains(q)
                    || b.created_by.to_lowercase().contains(q)
                    || b.created_at.to_lowercase().contains(q)
                    || b.root_manifest.to_lowercase().contains(q)
                {
                    return true;
                }
                if b.reasons.iter().any(|r| r.to_lowercase().contains(q)) {
                    return true;
                }
                false
            })
            .collect::<Vec<_>>();
        bundles.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        if let Some(n) = limit {
            bundles.truncate(n);
        }

        if bundles.is_empty() {
            let lines = vec!["(empty)".to_string()];
            self.push_output(lines.clone());
            self.panel = Some(Panel::Bundles {
                title: "Bundles".to_string(),
                updated_at: now_ts(),
                scope,
                gate,
                filter,
                items: Vec::new(),
                selected: 0,
            });
            return;
        }

        let mut lines = Vec::new();
        for b in &bundles {
            let bid = b.id.chars().take(8).collect::<String>();
            let tag = if b.promotable {
                "promotable"
            } else {
                "blocked"
            };
            let reasons = if b.reasons.is_empty() {
                "".to_string()
            } else {
                format!(" ({})", b.reasons.join(","))
            };
            lines.push(format!("{} {} {}{}", bid, b.created_at, tag, reasons));
        }
        self.push_output(lines);
        self.panel = Some(Panel::Bundles {
            title: "Bundles".to_string(),
            updated_at: now_ts(),
            scope,
            gate,
            filter,
            items: bundles,
            selected: 0,
        });
    }

    fn cmd_bundle(&mut self, args: &[String]) {
        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };
        let cfg = match self.remote_config() {
            Some(c) => c,
            None => return,
        };

        let mut scope: Option<String> = None;
        let mut gate: Option<String> = None;
        let mut pubs: Vec<String> = Vec::new();

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--scope" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --scope".to_string());
                        return;
                    }
                    scope = Some(args[i].clone());
                }
                "--gate" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --gate".to_string());
                        return;
                    }
                    gate = Some(args[i].clone());
                }
                "--publication" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --publication".to_string());
                        return;
                    }
                    pubs.push(args[i].clone());
                }
                a => {
                    self.push_error(format!("unknown arg: {}", a));
                    return;
                }
            }
            i += 1;
        }

        let scope = scope.unwrap_or(cfg.scope);
        let gate = gate.unwrap_or(cfg.gate);

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

    fn cmd_approve(&mut self, args: &[String]) {
        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };
        let mut bundle_id: Option<String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--bundle-id" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --bundle-id".to_string());
                        return;
                    }
                    bundle_id = Some(args[i].clone());
                }
                a => {
                    self.push_error(format!("unknown arg: {}", a));
                    return;
                }
            }
            i += 1;
        }

        let Some(bundle_id) = bundle_id else {
            self.push_error("usage: approve --bundle-id <id>".to_string());
            return;
        };

        match client.approve_bundle(&bundle_id) {
            Ok(_) => self.push_output(vec![format!("approved {}", bundle_id)]),
            Err(err) => self.push_error(format!("approve: {:#}", err)),
        }
    }

    fn cmd_promote(&mut self, args: &[String]) {
        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };

        let mut bundle_id: Option<String> = None;
        let mut to_gate: Option<String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--bundle-id" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --bundle-id".to_string());
                        return;
                    }
                    bundle_id = Some(args[i].clone());
                }
                "--to-gate" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --to-gate".to_string());
                        return;
                    }
                    to_gate = Some(args[i].clone());
                }
                a => {
                    self.push_error(format!("unknown arg: {}", a));
                    return;
                }
            }
            i += 1;
        }

        let Some(bundle_id) = bundle_id else {
            self.push_error("usage: promote --bundle-id <id> [--to-gate <id>]".to_string());
            return;
        };

        let to_gate = match to_gate {
            Some(g) => g,
            None => {
                // Convenience: if exactly one downstream gate, use it.
                let cfg = match self.remote_config() {
                    Some(c) => c,
                    None => return,
                };
                let graph = match client.get_gate_graph() {
                    Ok(g) => g,
                    Err(err) => {
                        self.push_error(format!("get gate graph: {:#}", err));
                        return;
                    }
                };
                let mut next = graph
                    .gates
                    .iter()
                    .filter(|g| g.upstream.iter().any(|u| u == &cfg.gate))
                    .map(|g| g.id.clone())
                    .collect::<Vec<_>>();
                next.sort();
                if next.len() == 1 {
                    next[0].clone()
                } else {
                    self.push_error(
                        "missing --to-gate and could not infer a unique downstream gate"
                            .to_string(),
                    );
                    return;
                }
            }
        };

        match client.promote_bundle(&bundle_id, &to_gate) {
            Ok(_) => self.push_output(vec![format!("promoted {} -> {}", bundle_id, to_gate)]),
            Err(err) => self.push_error(format!("promote: {:#}", err)),
        }
    }

    fn cmd_superpositions(&mut self, args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };
        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };

        let mut bundle_id: Option<String> = None;
        let mut filter: Option<String> = None;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--bundle-id" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --bundle-id".to_string());
                        return;
                    }
                    bundle_id = Some(args[i].clone());
                }
                "--filter" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --filter".to_string());
                        return;
                    }
                    filter = Some(args[i].clone());
                }
                a => {
                    self.push_error(format!("unknown arg: {}", a));
                    return;
                }
            }
            i += 1;
        }

        let Some(bundle_id) = bundle_id else {
            self.push_error("usage: superpositions --bundle-id <id>".to_string());
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

        let filter_lc = filter.as_ref().map(|s| s.to_lowercase());
        let mut items = variants
            .iter()
            .map(|(p, vs)| (p.clone(), vs.len()))
            .collect::<Vec<_>>();
        items.sort_by(|a, b| a.0.cmp(&b.0));
        if let Some(q) = filter_lc.as_deref() {
            items.retain(|(p, _)| p.to_lowercase().contains(q));
        }

        if items.is_empty() {
            let lines = vec!["(no superpositions)".to_string()];
            self.push_output(lines.clone());
            self.panel = Some(Panel::Superpositions {
                title: "Superpositions".to_string(),
                updated_at: now_ts(),
                bundle_id,
                filter,
                root_manifest: root,
                variants,
                decisions,
                validation,
                items: Vec::new(),
                selected: 0,
            });
            return;
        }
        let mut lines = Vec::new();
        for (p, n) in &items {
            lines.push(format!("{} (variants: {})", p, n));
        }
        self.push_output(lines);
        self.panel = Some(Panel::Superpositions {
            title: "Superpositions".to_string(),
            updated_at: now_ts(),
            bundle_id,
            filter,
            root_manifest: root,
            variants,
            decisions,
            validation,
            items,
            selected: 0,
        });
    }
}

fn score_match(q: &str, candidate: &str) -> i32 {
    let q = q.to_lowercase();
    let c = candidate.to_lowercase();
    if c == q {
        return 100;
    }
    if c.starts_with(&q) {
        return 50 - (c.len() as i32 - q.len() as i32);
    }
    if c.contains(&q) {
        return 10;
    }
    0
}

fn tokenize(input: &str) -> Result<Vec<String>> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut escape = false;

    for ch in input.chars() {
        if escape {
            cur.push(ch);
            escape = false;
            continue;
        }

        match ch {
            '\\' => {
                escape = true;
            }
            '"' => {
                in_quotes = !in_quotes;
            }
            c if c.is_whitespace() && !in_quotes => {
                if !cur.is_empty() {
                    out.push(cur);
                    cur = String::new();
                }
            }
            c => {
                cur.push(c);
            }
        }
    }

    if escape {
        anyhow::bail!("dangling escape");
    }
    if in_quotes {
        anyhow::bail!("unterminated quote");
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    Ok(out)
}

fn run_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| draw(f, app)).context("draw")?;
        if app.quit {
            return Ok(());
        }

        if event::poll(Duration::from_millis(50)).context("poll")? {
            match event::read().context("read event")? {
                Event::Key(k) if k.kind == KeyEventKind::Press => handle_key(app, k),
                _ => {}
            }
        }
    }
}

fn handle_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => {
            app.quit = true;
        }

        KeyCode::Esc => {
            if !app.input.buf.is_empty() {
                app.input.clear();
                app.recompute_suggestions();
            } else if app.panel.is_some() {
                app.clear_panel();
                app.push_output(vec!["panel closed".to_string()]);
            } else {
                app.quit = true;
            }
        }

        KeyCode::Tab => {
            if app.input.buf.is_empty() {
                app.mode = match app.mode {
                    Mode::Local => Mode::Remote,
                    Mode::Remote => Mode::Local,
                };
                app.push_output(vec![format!("switched to {:?} mode", app.mode)]);
            } else if !app.suggestions.is_empty() {
                app.suggestion_selected =
                    (app.suggestion_selected + 1) % app.suggestions.len().max(1);
                app.apply_selected_suggestion();
            }
        }

        KeyCode::Enter => {
            if app.input.buf.is_empty()
                && let Some(p) = &app.panel
                && let Some(idx) = p.selected()
            {
                match p {
                    Panel::Inbox { items, .. } => {
                        let id = &items[idx].id;
                        app.input.set(format!("bundle --publication {} ", id));
                        app.recompute_suggestions();
                        return;
                    }
                    Panel::Bundles { items, .. } => {
                        let id = &items[idx].id;
                        app.input.set(format!("superpositions --bundle-id {} ", id));
                        app.recompute_suggestions();
                        return;
                    }
                    Panel::Superpositions {
                        bundle_id, items, ..
                    } => {
                        let (path, _) = &items[idx];
                        app.input.set(format!(
                            "resolve pick --bundle-id {} --path \"{}\" --variant 1 ",
                            bundle_id, path
                        ));
                        app.recompute_suggestions();
                        return;
                    }
                    Panel::Status { .. } => {}
                }
            }

            app.run_current_input();
        }

        KeyCode::Up => {
            if app.input.buf.is_empty()
                && let Some(p) = &mut app.panel
            {
                p.move_up();
                return;
            }
            app.input.history_up();
            app.recompute_suggestions();
        }
        KeyCode::Down => {
            if app.input.buf.is_empty()
                && let Some(p) = &mut app.panel
            {
                p.move_down();
                return;
            }
            app.input.history_down();
            app.recompute_suggestions();
        }

        KeyCode::Left => {
            app.input.move_left();
        }
        KeyCode::Right => {
            app.input.move_right();
        }
        KeyCode::Backspace => {
            app.input.backspace();
            app.recompute_suggestions();
        }
        KeyCode::Delete => {
            app.input.delete();
            app.recompute_suggestions();
        }

        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input.clear();
            app.recompute_suggestions();
        }

        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input.history_up();
            app.recompute_suggestions();
        }

        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input.history_down();
            app.recompute_suggestions();
        }

        KeyCode::Char(c)
            if key.modifiers.contains(KeyModifiers::ALT) && app.input.buf.is_empty() =>
        {
            let selected = app.panel.as_ref().and_then(|p| p.selected());
            if let Some(idx) = selected {
                let is_superpositions =
                    matches!(app.panel.as_ref(), Some(Panel::Superpositions { .. }));

                if is_superpositions {
                    if c.is_ascii_digit() {
                        let n = c.to_digit(10).unwrap_or(0) as usize;
                        // Alt+0 clears; Alt+1..9 selects variant.
                        if n == 0 {
                            superpositions_clear_decision(app);
                        } else {
                            superpositions_pick_variant(app, n - 1);
                        }
                        return;
                    }

                    if c == 'f' {
                        superpositions_jump_next_invalid(app);
                        return;
                    }

                    if c == 'n' {
                        superpositions_jump_next_missing(app);
                        return;
                    }
                }

                let p = app.panel.as_ref().expect("panel exists if selected exists");

                let mut prefill = None;
                match (p, c) {
                    (Panel::Inbox { items, .. }, 'b') => {
                        prefill = Some(format!("bundle --publication {} ", items[idx].id));
                    }
                    (Panel::Bundles { items, .. }, 'a') => {
                        prefill = Some(format!("approve --bundle-id {} ", items[idx].id));
                    }
                    (Panel::Bundles { items, .. }, 'p') => {
                        prefill = Some(format!("promote --bundle-id {} ", items[idx].id));
                    }
                    (Panel::Bundles { items, .. }, 's') => {
                        prefill = Some(format!("superpositions --bundle-id {} ", items[idx].id));
                    }
                    (
                        Panel::Superpositions {
                            bundle_id, items, ..
                        },
                        'p',
                    ) => {
                        let (path, _) = &items[idx];
                        prefill = Some(format!(
                            "resolve pick --bundle-id {} --path \"{}\" --variant 1 ",
                            bundle_id, path
                        ));
                    }
                    (Panel::Superpositions { bundle_id, .. }, 'i') => {
                        prefill = Some(format!("resolve init --bundle-id {} ", bundle_id));
                    }
                    (Panel::Superpositions { bundle_id, .. }, 'v') => {
                        prefill = Some(format!("resolve validate --bundle-id {} ", bundle_id));
                    }
                    (Panel::Superpositions { bundle_id, .. }, 'a') => {
                        prefill = Some(format!("resolve apply --bundle-id {} ", bundle_id));
                    }
                    (Panel::Superpositions { bundle_id, .. }, 'u') => {
                        prefill = Some(format!(
                            "resolve apply --bundle-id {} --publish ",
                            bundle_id
                        ));
                    }
                    _ => {}
                }

                if let Some(prefill) = prefill {
                    app.input.set(prefill);
                    app.recompute_suggestions();
                }
            }
        }

        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input.insert_char(c);
            app.recompute_suggestions();
        }

        _ => {}
    }
}

fn superpositions_clear_decision(app: &mut App) {
    let Some(ws) = app.require_workspace() else {
        return;
    };

    let (bundle_id, root_manifest, path) = match app.panel.as_ref() {
        Some(Panel::Superpositions {
            bundle_id,
            root_manifest,
            items,
            selected,
            ..
        }) => {
            if items.is_empty() {
                app.push_error("no selected superposition".to_string());
                return;
            }
            let idx = (*selected).min(items.len().saturating_sub(1));
            let path = items[idx].0.clone();
            (bundle_id.clone(), root_manifest.clone(), path)
        }
        _ => return,
    };

    // Load or init resolution.
    let mut res = if ws.store.has_resolution(&bundle_id) {
        match ws.store.get_resolution(&bundle_id) {
            Ok(r) => r,
            Err(err) => {
                app.push_error(format!("load resolution: {:#}", err));
                return;
            }
        }
    } else {
        Resolution {
            version: 2,
            bundle_id: bundle_id.clone(),
            root_manifest: root_manifest.clone(),
            created_at: now_ts(),
            decisions: std::collections::BTreeMap::new(),
        }
    };

    if res.root_manifest != root_manifest {
        app.push_error("resolution root_manifest mismatch".to_string());
        return;
    }
    if res.version == 1 {
        res.version = 2;
    }

    res.decisions.remove(&path);
    if let Err(err) = ws.store.put_resolution(&res) {
        app.push_error(format!("write resolution: {:#}", err));
        return;
    }

    {
        let Some(Panel::Superpositions {
            root_manifest,
            decisions,
            validation,
            updated_at,
            ..
        }) = app.panel.as_mut()
        else {
            return;
        };

        decisions.remove(&path);
        *validation = validate_resolution(&ws.store, root_manifest, decisions).ok();
        *updated_at = now_ts();
    }

    app.push_output(vec![format!("cleared decision for {}", path)]);
}

fn superpositions_pick_variant(app: &mut App, variant_index: usize) {
    let Some(ws) = app.require_workspace() else {
        return;
    };

    let (bundle_id, root_manifest, path, key, variants_len) = match app.panel.as_ref() {
        Some(Panel::Superpositions {
            bundle_id,
            root_manifest,
            variants,
            items,
            selected,
            ..
        }) => {
            if items.is_empty() {
                app.push_error("no selected superposition".to_string());
                return;
            }
            let idx = (*selected).min(items.len().saturating_sub(1));
            let path = items[idx].0.clone();
            let Some(vs) = variants.get(&path) else {
                app.push_error("variants not loaded".to_string());
                return;
            };
            let variants_len = vs.len();
            let Some(v) = vs.get(variant_index) else {
                app.push_error(format!("variant out of range (variants: {})", variants_len));
                return;
            };
            (
                bundle_id.clone(),
                root_manifest.clone(),
                path,
                v.key(),
                variants_len,
            )
        }
        _ => return,
    };

    // Load or init resolution.
    let mut res = if ws.store.has_resolution(&bundle_id) {
        match ws.store.get_resolution(&bundle_id) {
            Ok(r) => r,
            Err(err) => {
                app.push_error(format!("load resolution: {:#}", err));
                return;
            }
        }
    } else {
        Resolution {
            version: 2,
            bundle_id: bundle_id.clone(),
            root_manifest: root_manifest.clone(),
            created_at: now_ts(),
            decisions: std::collections::BTreeMap::new(),
        }
    };

    if res.root_manifest != root_manifest {
        app.push_error("resolution root_manifest mismatch".to_string());
        return;
    }
    if res.version == 1 {
        res.version = 2;
    }

    let decision = ResolutionDecision::Key(key);
    res.decisions.insert(path.clone(), decision.clone());
    if let Err(err) = ws.store.put_resolution(&res) {
        app.push_error(format!("write resolution: {:#}", err));
        return;
    }

    {
        let Some(Panel::Superpositions {
            root_manifest,
            decisions,
            validation,
            updated_at,
            ..
        }) = app.panel.as_mut()
        else {
            return;
        };

        decisions.insert(path.clone(), decision);
        *validation = validate_resolution(&ws.store, root_manifest, decisions).ok();
        *updated_at = now_ts();
    }

    app.push_output(vec![format!(
        "picked variant #{} for {} (variants: {})",
        variant_index + 1,
        path,
        variants_len
    )]);
}

fn superpositions_jump_next_missing(app: &mut App) {
    let next = match app.panel.as_ref() {
        Some(Panel::Superpositions {
            items,
            selected,
            decisions,
            ..
        }) => {
            if items.is_empty() {
                return;
            }
            let start = (*selected).min(items.len().saturating_sub(1));
            (1..=items.len()).find_map(|off| {
                let idx = (start + off) % items.len();
                let path = &items[idx].0;
                if !decisions.contains_key(path) {
                    Some(idx)
                } else {
                    None
                }
            })
        }
        _ => return,
    };

    if let Some(idx) = next {
        if let Some(Panel::Superpositions {
            selected,
            updated_at,
            ..
        }) = app.panel.as_mut()
        {
            *selected = idx;
            *updated_at = now_ts();
        }
        app.push_output(vec!["jumped to missing".to_string()]);
    } else {
        app.push_output(vec!["no missing decisions".to_string()]);
    }
}

fn superpositions_jump_next_invalid(app: &mut App) {
    let next = match app.panel.as_ref() {
        Some(Panel::Superpositions {
            items,
            selected,
            validation,
            ..
        }) => {
            if items.is_empty() {
                return;
            }

            let Some(vr) = validation.as_ref() else {
                return;
            };

            let mut invalid = std::collections::HashSet::new();
            for d in &vr.invalid_keys {
                invalid.insert(d.path.as_str());
            }
            for d in &vr.out_of_range {
                invalid.insert(d.path.as_str());
            }

            let start = (*selected).min(items.len().saturating_sub(1));
            (1..=items.len()).find_map(|off| {
                let idx = (start + off) % items.len();
                let path = items[idx].0.as_str();
                if invalid.contains(path) {
                    Some(idx)
                } else {
                    None
                }
            })
        }
        _ => return,
    };

    if let Some(idx) = next {
        if let Some(Panel::Superpositions {
            selected,
            updated_at,
            ..
        }) = app.panel.as_mut()
        {
            *selected = idx;
            *updated_at = now_ts();
        }
        app.push_output(vec!["jumped to invalid".to_string()]);
    } else {
        app.push_output(vec!["no invalid decisions".to_string()]);
    }
}

fn draw(frame: &mut ratatui::Frame, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(if app.suggestions.is_empty() { 0 } else { 6 }),
            Constraint::Length(3),
        ])
        .split(area);

    // Header
    let ws = app
        .workspace
        .as_ref()
        .map(|w| w.root.display().to_string())
        .or_else(|| app.workspace_err.clone())
        .unwrap_or_else(|| "(no workspace)".to_string());

    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            "Converge",
            Style::default().fg(Color::Black).bg(Color::White),
        ),
        Span::raw("  "),
        Span::styled(app.prompt(), Style::default().fg(Color::Yellow)),
        Span::raw("  "),
        Span::raw(ws),
    ]))
    .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(header, chunks[0]);

    // Main area: scrollback + optional panel
    let (scroll_area, panel_area) = if app.panel.is_some() {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(chunks[1]);
        (cols[0], Some(cols[1]))
    } else {
        (chunks[1], None)
    };

    // Scrollback
    let mut lines = Vec::new();
    for e in &app.scroll {
        let prefix_style = match e.kind {
            EntryKind::Command => Style::default().fg(Color::Cyan),
            EntryKind::Output => Style::default().fg(Color::White),
            EntryKind::Error => Style::default().fg(Color::Red),
        };
        let tag = match e.kind {
            EntryKind::Command => ">",
            EntryKind::Output => " ",
            EntryKind::Error => "!",
        };

        for (i, l) in e.lines.iter().enumerate() {
            if i == 0 {
                lines.push(Line::from(vec![
                    Span::styled(format!("{} ", tag), prefix_style),
                    Span::styled(format!("{} ", e.ts), Style::default().fg(Color::Gray)),
                    Span::raw(l.as_str()),
                ]));
            } else {
                lines.push(Line::from(vec![Span::raw("  "), Span::raw(l.as_str())]));
            }
        }
    }
    if lines.is_empty() {
        lines.push(Line::from(""));
    }

    // Show the most recent lines that fit.
    let max = chunks[1].height as usize;
    if max > 0 && lines.len() > max {
        lines = lines.split_off(lines.len() - max);
    }

    let body = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(body, scroll_area);

    // Panel
    if let (Some(p), Some(area)) = (&app.panel, panel_area) {
        draw_panel(frame, app, p, area);
    }

    // Suggestions
    if !app.suggestions.is_empty() {
        let mut s_lines = Vec::new();
        s_lines.push(Line::from(Span::styled(
            "Suggestions",
            Style::default().fg(Color::Gray),
        )));
        for (i, s) in app.suggestions.iter().enumerate() {
            let sel = i
                == app
                    .suggestion_selected
                    .min(app.suggestions.len().saturating_sub(1));
            let style = if sel {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };
            s_lines.push(Line::from(vec![
                Span::styled(format!("{: <10}", s.name), style.fg(Color::Yellow)),
                Span::styled(s.help, style.fg(Color::White)),
            ]));
        }
        let sugg =
            Paragraph::new(s_lines).block(Block::default().borders(Borders::TOP | Borders::BOTTOM));
        frame.render_widget(sugg, chunks[2]);
    }

    // Input
    let prompt = app.prompt();
    let buf = &app.input.buf;
    let input_line = Line::from(vec![
        Span::styled(prompt, Style::default().fg(Color::Yellow)),
        Span::raw(" "),
        Span::raw(buf.as_str()),
    ]);
    let input = Paragraph::new(input_line).block(Block::default().borders(Borders::TOP));
    frame.render_widget(input, chunks[3]);

    // Cursor
    let x = prompt.len() as u16 + 1 + app.input.cursor as u16;
    let y = chunks[3].y + 1;
    frame.set_cursor_position((chunks[3].x + x, y));
}

fn draw_panel(frame: &mut ratatui::Frame, app: &App, panel: &Panel, area: ratatui::layout::Rect) {
    let header = Line::from(vec![
        Span::styled(panel.title(), Style::default().fg(Color::Yellow)),
        Span::raw("  "),
        Span::styled(panel.updated_at(), Style::default().fg(Color::Gray)),
    ]);

    let outer = Block::default().borders(Borders::LEFT).title(header);
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    match panel {
        Panel::Status { lines, .. } => {
            let mut plines = Vec::new();
            for l in lines {
                plines.push(Line::from(l.as_str()));
            }
            let max = inner.height as usize;
            if max > 0 && plines.len() > max {
                plines = plines.split_off(plines.len() - max);
            }
            frame.render_widget(Paragraph::new(plines).wrap(Wrap { trim: false }), inner);
        }

        Panel::Inbox {
            scope,
            gate,
            filter,
            items,
            selected,
            ..
        } => {
            let parts = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
                .split(inner);

            let mut state = ListState::default();
            if !items.is_empty() {
                state.select(Some((*selected).min(items.len().saturating_sub(1))));
            }

            let mut rows = Vec::new();
            for p in items {
                let rid = p.id.chars().take(8).collect::<String>();
                let sid = p.snap_id.chars().take(8).collect::<String>();
                let res = if p.resolution.is_some() {
                    " resolved"
                } else {
                    ""
                };
                rows.push(ListItem::new(format!("{} {}{}", rid, sid, res)));
            }
            if rows.is_empty() {
                rows.push(ListItem::new("(empty)"));
            }

            let list = List::new(rows)
                .block(Block::default().borders(Borders::BOTTOM).title(format!(
                    "scope={} gate={}{} (Enter: bundle, Alt+b: bundle)",
                    scope,
                    gate,
                    filter
                        .as_ref()
                        .map(|f| format!(" filter={}", f))
                        .unwrap_or_default()
                )))
                .highlight_style(Style::default().bg(Color::DarkGray));
            frame.render_stateful_widget(list, parts[0], &mut state);

            let details = if items.is_empty() {
                vec![Line::from("(no selection)")]
            } else {
                let idx = (*selected).min(items.len().saturating_sub(1));
                let p = &items[idx];
                let mut out = Vec::new();
                out.push(Line::from(format!("id: {}", p.id)));
                out.push(Line::from(format!("snap: {}", p.snap_id)));
                out.push(Line::from(format!("publisher: {}", p.publisher)));
                out.push(Line::from(format!("created_at: {}", p.created_at)));
                if let Some(r) = &p.resolution {
                    out.push(Line::from(""));
                    out.push(Line::from("resolution:"));
                    out.push(Line::from(format!("  bundle_id: {}", r.bundle_id)));
                }
                out
            };
            frame.render_widget(Paragraph::new(details).wrap(Wrap { trim: false }), parts[1]);
        }

        Panel::Bundles {
            scope,
            gate,
            filter,
            items,
            selected,
            ..
        } => {
            let parts = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
                .split(inner);

            let mut state = ListState::default();
            if !items.is_empty() {
                state.select(Some((*selected).min(items.len().saturating_sub(1))));
            }

            let mut rows = Vec::new();
            for b in items {
                let bid = b.id.chars().take(8).collect::<String>();
                let tag = if b.promotable {
                    "promotable"
                } else {
                    "blocked"
                };
                rows.push(ListItem::new(format!("{} {}", bid, tag)));
            }
            if rows.is_empty() {
                rows.push(ListItem::new("(empty)"));
            }

            let list = List::new(rows)
                .block(Block::default().borders(Borders::BOTTOM).title(format!(
                    "scope={} gate={}{} (Enter: superpositions, Alt+a approve, Alt+p promote)",
                    scope,
                    gate,
                    filter
                        .as_ref()
                        .map(|f| format!(" filter={}", f))
                        .unwrap_or_default()
                )))
                .highlight_style(Style::default().bg(Color::DarkGray));
            frame.render_stateful_widget(list, parts[0], &mut state);

            let details = if items.is_empty() {
                vec![Line::from("(no selection)")]
            } else {
                let idx = (*selected).min(items.len().saturating_sub(1));
                let b = &items[idx];
                let mut out = Vec::new();
                out.push(Line::from(format!("id: {}", b.id)));
                out.push(Line::from(format!("created_at: {}", b.created_at)));
                out.push(Line::from(format!("created_by: {}", b.created_by)));
                out.push(Line::from(format!("promotable: {}", b.promotable)));
                if !b.reasons.is_empty() {
                    out.push(Line::from(format!("reasons: {}", b.reasons.join(", "))));
                }
                out
            };
            frame.render_widget(Paragraph::new(details).wrap(Wrap { trim: false }), parts[1]);
        }

        Panel::Superpositions {
            bundle_id,
            filter,
            root_manifest,
            variants,
            decisions,
            validation,
            items,
            selected,
            ..
        } => {
            let parts = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
                .split(inner);

            let mut state = ListState::default();
            if !items.is_empty() {
                state.select(Some((*selected).min(items.len().saturating_sub(1))));
            }

            let mut rows = Vec::new();
            for (p, n) in items {
                let mark = match decisions.get(p) {
                    None => " ".to_string(),
                    Some(ResolutionDecision::Index(i)) => {
                        let n = (*i as usize) + 1;
                        if n <= 9 {
                            format!("{}", n)
                        } else {
                            "*".to_string()
                        }
                    }
                    Some(ResolutionDecision::Key(k)) => {
                        let idx = variants
                            .get(p)
                            .and_then(|vs| vs.iter().position(|v| v.key() == *k));
                        match idx {
                            Some(i) if i < 9 => format!("{}", i + 1),
                            Some(_) => "*".to_string(),
                            None => "!".to_string(),
                        }
                    }
                };

                rows.push(ListItem::new(format!("[{}] {} ({})", mark, p, n)));
            }
            if rows.is_empty() {
                rows.push(ListItem::new("(none)"));
            }
            let list = List::new(rows)
                .block(Block::default().borders(Borders::BOTTOM).title(format!(
                    "bundle={}{}{} (Alt+1..9 pick, Alt+0 clear, Alt+n next missing, Alt+f next invalid)",
                    bundle_id.chars().take(8).collect::<String>(),
                    filter
                        .as_ref()
                        .map(|f| format!(" filter={}", f))
                        .unwrap_or_default(),
                    validation
                        .as_ref()
                        .map(|v| {
                            format!(
                                " missing={} invalid={}",
                                v.missing.len(),
                                v.invalid_keys.len() + v.out_of_range.len()
                            )
                        })
                        .unwrap_or_default()
                )))
                .highlight_style(Style::default().bg(Color::DarkGray));
            frame.render_stateful_widget(list, parts[0], &mut state);

            let details = if items.is_empty() {
                vec![Line::from("(no selection)")]
            } else {
                let idx = (*selected).min(items.len().saturating_sub(1));
                let (p, n) = &items[idx];
                let mut out = Vec::new();
                out.push(Line::from(format!("path: {}", p)));
                out.push(Line::from(format!("variants: {}", n)));
                out.push(Line::from(format!(
                    "root_manifest: {}",
                    root_manifest.as_str()
                )));

                if let Some(vr) = validation {
                    out.push(Line::from(""));
                    out.push(Line::from(format!(
                        "validation: {}",
                        if vr.ok { "ok" } else { "invalid" }
                    )));
                    if !vr.missing.is_empty() {
                        out.push(Line::from(format!("missing: {}", vr.missing.len())));
                    }
                    if !vr.invalid_keys.is_empty() {
                        out.push(Line::from(format!(
                            "invalid_keys: {}",
                            vr.invalid_keys.len()
                        )));
                    }
                    if !vr.out_of_range.is_empty() {
                        out.push(Line::from(format!(
                            "out_of_range: {}",
                            vr.out_of_range.len()
                        )));
                    }
                    if !vr.extraneous.is_empty() {
                        out.push(Line::from(format!("extraneous: {}", vr.extraneous.len())));
                    }
                }

                let chosen = decisions.get(p);
                if let Some(chosen) = chosen {
                    out.push(Line::from(""));
                    out.push(Line::from(format!(
                        "chosen: {}",
                        match chosen {
                            ResolutionDecision::Index(i) => format!("index {}", i),
                            ResolutionDecision::Key(_) => "key".to_string(),
                        }
                    )));
                }

                if let Some(vs) = variants.get(p) {
                    out.push(Line::from(""));
                    out.push(Line::from("variants:"));
                    for (i, v) in vs.iter().enumerate() {
                        let key_json =
                            serde_json::to_string(&v.key()).unwrap_or_else(|_| "<key>".to_string());
                        out.push(Line::from(format!("  #{} source={}", i + 1, v.source)));
                        out.push(Line::from(format!("    key={}", key_json)));
                        match &v.kind {
                            crate::model::SuperpositionVariantKind::File { blob, mode, size } => {
                                out.push(Line::from(format!(
                                    "    file blob={} mode={:#o} size={}",
                                    blob.as_str(),
                                    mode,
                                    size
                                )));
                            }
                            crate::model::SuperpositionVariantKind::Dir { manifest } => {
                                out.push(Line::from(format!(
                                    "    dir manifest={}",
                                    manifest.as_str()
                                )));
                            }
                            crate::model::SuperpositionVariantKind::Symlink { target } => {
                                out.push(Line::from(format!("    symlink target={}", target)));
                            }
                            crate::model::SuperpositionVariantKind::Tombstone => {
                                out.push(Line::from("    tombstone"));
                            }
                        }
                    }
                }

                out
            };
            frame.render_widget(Paragraph::new(details).wrap(Wrap { trim: false }), parts[1]);
        }
    }

    // Help footer within panel when input is empty.
    if app.input.buf.is_empty() {
        let _ = app;
    }
}
