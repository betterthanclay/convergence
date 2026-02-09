use super::super::*;

mod init;
mod set;
mod show;

pub(super) fn handle_gates_command(ws: &Workspace, command: GateGraphCommands) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote.clone(), token)?;

    match command {
        GateGraphCommands::Show { json } => show::show_gate_graph(&client, json),
        GateGraphCommands::Set { file, json } => set::set_gate_graph(&client, file, json),
        GateGraphCommands::Init { apply, json } => init::init_gate_graph(&client, apply, json),
    }
}
