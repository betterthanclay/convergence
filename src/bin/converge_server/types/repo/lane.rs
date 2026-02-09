use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Lane {
    pub(crate) id: String,
    pub(crate) members: HashSet<String>,

    #[serde(default)]
    pub(crate) member_user_ids: HashSet<String>,

    #[serde(default)]
    pub(crate) heads: HashMap<String, LaneHead>,

    // Retention roots for unpublished collaboration. We keep a bounded history of head
    // updates so the server can GC aggressively without losing recent WIP context.
    #[serde(default)]
    pub(crate) head_history: HashMap<String, Vec<LaneHead>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct LaneHead {
    pub(crate) snap_id: String,
    pub(crate) updated_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) client_id: Option<String>,
}

pub(crate) const LANE_HEAD_HISTORY_KEEP_LAST: usize = 5;
