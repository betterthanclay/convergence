#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BootstrapResponse {
    pub user: RemoteUser,
    pub token: CreateTokenResponse,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct WhoAmI {
    pub user: String,
    pub user_id: String,
    pub admin: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TokenView {
    pub id: String,
    pub label: Option<String>,
    pub created_at: String,
    pub last_used_at: Option<String>,
    pub revoked_at: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateTokenResponse {
    pub id: String,
    pub token: String,
    pub created_at: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RemoteUser {
    pub id: String,
    pub handle: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub admin: bool,
    pub created_at: String,
}
