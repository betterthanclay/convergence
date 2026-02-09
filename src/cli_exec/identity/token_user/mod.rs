use super::*;

mod token;
mod user;

pub(crate) fn handle_token_command(ws: &Workspace, command: TokenCommands) -> Result<()> {
    token::handle_token_command(ws, command)
}

pub(crate) fn handle_user_command(ws: &Workspace, command: UserCommands) -> Result<()> {
    user::handle_user_command(ws, command)
}
