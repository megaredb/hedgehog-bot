use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct RefreshAuthDataRequest {
    pub device_id: String,
    pub device_os: String,
    pub grant_type: String,
    pub refresh_token: String,
}

#[derive(Deserialize, Debug)]
pub struct RefreshAuthDataResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u128,
}
