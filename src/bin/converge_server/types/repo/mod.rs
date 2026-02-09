use super::*;

mod gate_graph;
mod lane;
mod publication_flow;

pub(crate) use self::gate_graph::{Gate, GateDef, GateGraph};
pub(crate) use self::lane::{LANE_HEAD_HISTORY_KEEP_LAST, Lane, LaneHead};
pub(crate) use self::publication_flow::{
    Bundle, Promotion, Publication, PublicationResolution, Release,
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Repo {
    pub(crate) id: String,
    pub(crate) owner: String,

    #[serde(default)]
    pub(crate) owner_user_id: Option<String>,

    pub(crate) readers: HashSet<String>,

    #[serde(default)]
    pub(crate) reader_user_ids: HashSet<String>,

    pub(crate) publishers: HashSet<String>,

    #[serde(default)]
    pub(crate) publisher_user_ids: HashSet<String>,

    pub(crate) lanes: HashMap<String, Lane>,

    pub(crate) gate_graph: GateGraph,
    pub(crate) scopes: HashSet<String>,

    pub(crate) snaps: HashSet<String>,
    pub(crate) publications: Vec<Publication>,

    pub(crate) bundles: Vec<Bundle>,

    #[serde(default)]
    pub(crate) pinned_bundles: HashSet<String>,

    pub(crate) promotions: Vec<Promotion>,
    pub(crate) promotion_state: HashMap<String, HashMap<String, String>>,

    #[serde(default)]
    pub(crate) releases: Vec<Release>,
}
