use url::Url;
use crate::domain::traits::value_object::ValueObject;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedirectUri {
    uri: Url
}

impl RedirectUri {
    pub fn new(uri_str: &str) -> Result<Self, String> {
        Url::parse(uri_str)
            .map(|url| Self { uri: url })
            .map_err(|_| "Invalid URI".into())
    }

    pub fn as_str(&self) -> &str {
        self.uri.as_str()
    }
}

impl ValueObject for RedirectUri {}