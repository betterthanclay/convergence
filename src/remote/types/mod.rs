//! DTOs and payload types for remote API requests/responses.

mod auth;
mod gate_graph;
mod publication_flow;
mod repo_lanes;
mod requests;

pub use self::auth::{BootstrapResponse, CreateTokenResponse, RemoteUser, TokenView, WhoAmI};
pub(crate) use self::gate_graph::GateGraphValidationError;
pub use self::gate_graph::{GateDef, GateGraph};
pub use self::publication_flow::{
    Bundle, MissingObjectsResponse, Pins, Promotion, Publication, PublicationResolution, Release,
};
pub use self::repo_lanes::{Lane, LaneHead, LaneMembers, Repo, RepoMembers};
pub(crate) use self::requests::{
    CreatePublicationRequest, CreateRepoRequest, MissingObjectsRequest, UpdateLaneHeadRequest,
};
