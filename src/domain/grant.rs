pub struct Grant {
    pub user_id: Uuid,
    pub client_id: String,
    pub scope: Vec<String>,
    pub code: Option<AuthorizationCode>,
    pub access_token: Option<AccessToken>,
    pub refresh_token: Option<RefreshToken>
}