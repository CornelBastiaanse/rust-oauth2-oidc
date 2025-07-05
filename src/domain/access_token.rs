pub struct AccessToken {
    pub id: Uuid,
    pub value: String,
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub scope: Vec<String>,
    pub expires_at: DateTime<Utc>,
    pub issued_at: DateTime<Utc>
}