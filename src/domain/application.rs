pub struct Application {
    pub client_id: String,
    pub client_secret_hash: Option<String>,
    pub name: String,
    pub redirect_uris: Vec<String>,
    pub client_type: ClientType,
    pub scopes: Vec<String>
}