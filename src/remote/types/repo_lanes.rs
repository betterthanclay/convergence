use std::collections::{HashMap, HashSet};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Repo {
    pub id: String,
    pub owner: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RepoMembers {
    pub owner: String,
    pub readers: Vec<String>,
    pub publishers: Vec<String>,

    #[serde(default)]
    pub owner_user_id: Option<String>,
    #[serde(default)]
    pub reader_user_ids: Vec<String>,
    #[serde(default)]
    pub publisher_user_ids: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LaneMembers {
    pub lane: String,
    pub members: Vec<String>,

    #[serde(default)]
    pub member_user_ids: Vec<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LaneHead {
    pub snap_id: String,
    pub updated_at: String,

    #[serde(default)]
    pub client_id: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Lane {
    pub id: String,
    pub members: HashSet<String>,

    #[serde(default)]
    pub heads: HashMap<String, LaneHead>,
}
