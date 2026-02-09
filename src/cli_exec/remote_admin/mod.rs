use super::*;

mod gate_graph;
mod remote_ops;

pub(super) fn handle_remote_command(ws: &Workspace, command: RemoteCommands) -> Result<()> {
    remote_ops::handle_remote_command(ws, command)
}

pub(super) fn handle_gates_command(ws: &Workspace, command: GateGraphCommands) -> Result<()> {
    gate_graph::handle_gates_command(ws, command)
}
