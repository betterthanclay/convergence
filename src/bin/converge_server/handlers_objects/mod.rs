use super::*;

mod blob;
mod manifest;
mod recipe;
mod snap;

pub(super) use self::blob::{get_blob, put_blob};
pub(super) use self::manifest::{get_manifest, put_manifest};
pub(super) use self::recipe::{get_recipe, put_recipe};
pub(super) use self::snap::{get_snap, put_snap};

#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct PutObjectQuery {
    #[serde(default)]
    allow_missing_blobs: bool,
}
