mod mode_defs;
mod root_defs;

pub(in crate::tui_shell) use self::mode_defs::{
    bundles_command_defs, gate_graph_command_defs, inbox_command_defs, lanes_command_defs,
    releases_command_defs, snaps_command_defs, superpositions_command_defs,
};
pub(in crate::tui_shell) use self::root_defs::{global_command_defs, root_command_defs};
