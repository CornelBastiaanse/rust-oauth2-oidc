pub struct RefreshToken {
    pub id: Uuid,
    pub value: String,
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub scope: Vec<String>,
    pub issued_at: DateTime<Utc>,
    pub revoked: bool
}