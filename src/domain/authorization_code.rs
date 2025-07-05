pub struct AuthorizationCode {
    pub code: String,
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub redirect_uri: String,
    pub scope: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub pkce_challenge: Option<String>,
    pub pkce_method: Option<String>
}