use super::types::{RootContext, UiMode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::tui_shell) enum PendingAction {
    Root { root_ctx: RootContext, cmd: String },
    Mode { mode: UiMode, cmd: String },
}

#[derive(Debug, Clone)]
pub(in crate::tui_shell) enum TextInputAction {
    ChunkingSet,
    WorkflowProfileSet,
    RetentionKeepLast,
    RetentionKeepDays,

    LoginUrl,
    LoginToken,
    LoginRepo,
    LoginScope,
    LoginGate,

    FetchKind,
    FetchId,
    FetchUser,
    FetchOptions,

    PublishStart,
    PublishSnap,
    PublishScope,
    PublishGate,
    PublishMeta,

    SyncStart,
    SyncLane,
    SyncClient,
    SyncSnap,

    ReleaseChannel,
    ReleaseNotes,

    ReleaseBundleId,

    PromoteToGate,
    PromoteBundleId,

    PinBundleId,
    PinAction,

    ApproveBundleId,
    SuperpositionsBundleId,

    MemberAction,
    MemberHandle,
    MemberRole,

    LaneMemberAction,
    LaneMemberLane,
    LaneMemberHandle,

    MoveFrom,
    MoveTo,

    BootstrapUrl,
    BootstrapToken,
    BootstrapHandle,
    BootstrapDisplayName,
    BootstrapRepo,
    BootstrapScope,
    BootstrapGate,

    GateGraphAddGateId,
    GateGraphAddGateName,
    GateGraphAddGateUpstream,
    GateGraphEditUpstream,
    GateGraphSetApprovals,

    BrowseQuery,
}

#[derive(Debug)]
pub(in crate::tui_shell) enum ModalKind {
    Viewer,
    SnapMessage {
        snap_id: String,
    },
    ConfirmAction {
        action: PendingAction,
    },
    TextInput {
        action: TextInputAction,
        prompt: String,
    },
}

#[derive(Debug)]
pub(in crate::tui_shell) struct Modal {
    pub(in crate::tui_shell) title: String,
    pub(in crate::tui_shell) lines: Vec<String>,
    pub(in crate::tui_shell) scroll: usize,

    pub(in crate::tui_shell) kind: ModalKind,
    pub(in crate::tui_shell) input: super::Input,
}
