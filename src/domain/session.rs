pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub client_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revoked: bool
}