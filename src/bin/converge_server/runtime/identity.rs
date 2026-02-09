use std::collections::HashMap;

use anyhow::{Context, Result};

use super::Args;
use crate::types::{AccessToken, User};

use crate::identity_store::{
    bootstrap_identity, load_identity_from_disk, persist_identity_to_disk,
};

pub(super) fn load_or_bootstrap_identity(
    args: &Args,
) -> Result<(HashMap<String, User>, HashMap<String, AccessToken>)> {
    let (mut users, mut tokens) =
        load_identity_from_disk(&args.data_dir).context("load identity")?;

    if users.is_empty() || tokens.is_empty() {
        if args.bootstrap_token.is_some() {
            if !(users.is_empty() && tokens.is_empty()) {
                anyhow::bail!(
                    "identity store inconsistent (users/tokens missing); remove {} to re-bootstrap",
                    args.data_dir.display()
                );
            }
        } else {
            let (u, t) = bootstrap_identity(&args.dev_user, &args.dev_token);
            users.insert(u.id.clone(), u);
            tokens.insert(t.id.clone(), t);
            persist_identity_to_disk(&args.data_dir, &users, &tokens)
                .context("persist identity")?;
        }
    }

    Ok((users, tokens))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use tempfile::tempdir;

    use super::*;

    fn args_with_data_dir(data_dir: PathBuf) -> Args {
        Args {
            addr: "127.0.0.1:0".parse().expect("parse socket addr"),
            addr_file: None,
            data_dir,
            db_url: None,
            bootstrap_token: None,
            dev_user: "dev".to_string(),
            dev_token: "dev-token".to_string(),
        }
    }

    #[test]
    fn load_or_bootstrap_identity_seeds_dev_identity_without_bootstrap_token() {
        let temp = tempdir().expect("create temp dir");
        let args = args_with_data_dir(temp.path().to_path_buf());

        let (users, tokens) = load_or_bootstrap_identity(&args).expect("load identity");

        assert_eq!(users.len(), 1);
        assert_eq!(tokens.len(), 1);
        assert!(users.values().any(|u| u.handle == "dev"));

        let users_path = crate::identity_store::identity_users_path(temp.path());
        let tokens_path = crate::identity_store::identity_tokens_path(temp.path());
        assert!(users_path.exists(), "users file should be persisted");
        assert!(tokens_path.exists(), "tokens file should be persisted");
    }

    #[test]
    fn load_or_bootstrap_identity_keeps_store_empty_with_bootstrap_token() {
        let temp = tempdir().expect("create temp dir");
        let mut args = args_with_data_dir(temp.path().to_path_buf());
        args.bootstrap_token = Some("bootstrap-secret".to_string());

        let (users, tokens) = load_or_bootstrap_identity(&args).expect("load identity");

        assert!(users.is_empty());
        assert!(tokens.is_empty());
    }
}
