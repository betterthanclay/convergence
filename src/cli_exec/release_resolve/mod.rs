use super::*;

mod release_cmd;
mod resolve_apply_validate;
mod resolve_init;
mod resolve_pick_clear_show;

pub(super) fn handle_release_command(ws: &Workspace, command: ReleaseCommands) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote, token)?;

    match command {
        ReleaseCommands::Create {
            channel,
            bundle_id,
            notes,
            json,
        } => release_cmd::handle_release_create(&client, channel, bundle_id, notes, json)?,
        ReleaseCommands::List { json } => release_cmd::handle_release_list(&client, json)?,
        ReleaseCommands::Show { channel, json } => {
            release_cmd::handle_release_show(&client, channel, json)?
        }
    }

    Ok(())
}

pub(super) fn handle_resolve_command(ws: &Workspace, command: ResolveCommands) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote.clone(), token)?;

    match command {
        ResolveCommands::Init {
            bundle_id,
            force,
            json,
        } => resolve_init::handle_resolve_init(ws, &client, bundle_id, force, json)?,
        ResolveCommands::Pick {
            bundle_id,
            path,
            variant,
            key,
            json,
        } => resolve_pick_clear_show::handle_resolve_pick(
            ws, &client, bundle_id, path, variant, key, json,
        )?,
        ResolveCommands::Clear {
            bundle_id,
            path,
            json,
        } => resolve_pick_clear_show::handle_resolve_clear(ws, bundle_id, path, json)?,
        ResolveCommands::Show { bundle_id, json } => {
            resolve_pick_clear_show::handle_resolve_show(ws, &client, bundle_id, json)?
        }
        ResolveCommands::Apply {
            bundle_id,
            message,
            publish,
            json,
        } => resolve_apply_validate::handle_resolve_apply(
            ws,
            &client,
            resolve_apply_validate::ResolveApplyInput {
                bundle_id,
                message,
                publish,
                json,
                scope: remote.scope.clone(),
                gate: remote.gate.clone(),
            },
        )?,
        ResolveCommands::Validate { bundle_id, json } => {
            resolve_apply_validate::handle_resolve_validate(ws, &client, bundle_id, json)?
        }
    }

    Ok(())
}
