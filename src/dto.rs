use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GetAccessTokenParams {
    grant_type: String,
    client_id: String,
    client_secret: String,
    refresh_token: String,
}

impl GetAccessTokenParams {
    pub fn new(client_id: String, client_secret: String, refresh_token: String) -> Self {
        Self {
            grant_type: "refresh_token".to_string(),
            client_id,
            client_secret,
            refresh_token,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub scope: String,
    pub expires_in: usize,
    pub token_type: String,
}
