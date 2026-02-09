use super::*;

pub(crate) fn handle_user_command(ws: &Workspace, command: UserCommands) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote, token)?;

    match command {
        UserCommands::List { json } => {
            let mut users = client.list_users()?;
            users.sort_by(|a, b| a.handle.cmp(&b.handle));
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&users).context("serialize users json")?
                );
            } else {
                for u in users {
                    let admin = if u.admin { " admin" } else { "" };
                    println!("{} {}{}", u.id, u.handle, admin);
                }
            }
        }
        UserCommands::Create {
            handle,
            display_name,
            admin,
            json,
        } => {
            let created = client.create_user(&handle, display_name, admin)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&created).context("serialize user create json")?
                );
            } else {
                println!("user_id: {}", created.id);
                println!("handle: {}", created.handle);
                println!("admin: {}", created.admin);
            }
        }
    }

    Ok(())
}
