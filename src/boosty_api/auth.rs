use std::{
    fmt::Display,
    fs::{read_to_string, write},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

const AUTH_CONFIG_FILE: &str = "auth.toml";

fn current_unix_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn default_device_os() -> String {
    "web".to_string()
}

fn default_grant_type() -> String {
    "refresh_token".to_string()
}

pub fn deserialize_expires_at<'a, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'a>,
{
    String::deserialize(deserializer).and_then(|value| {
        value
            .parse::<u128>()
            .map_err(|err| de::Error::custom(err.to_string()))
    })
}

pub fn serialize_expires_at<S>(value: &u128, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.to_string().serialize(serializer)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthData {
    #[serde(rename = "access-token")]
    pub access_token: String,
    #[serde(rename = "refresh-token")]
    pub refresh_token: String,
    #[serde(rename = "device-id")]
    pub device_id: String,
    #[serde(default = "default_device_os", skip)]
    pub device_os: String,
    #[serde(default = "default_grant_type", skip)]
    pub grant_type: String,
    #[serde(
        rename = "expires-at",
        serialize_with = "serialize_expires_at",
        deserialize_with = "deserialize_expires_at"
    )]
    pub expires_at: u128,
}

impl From<AuthData> for String {
    fn from(val: AuthData) -> Self {
        val.access_token.to_owned()
    }
}

impl Display for AuthData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.access_token)
    }
}

impl AuthData {
    pub fn load() -> Self {
        toml::from_str(
            &read_to_string(AUTH_CONFIG_FILE)
                .expect("Unable to find auth.toml in the current directory."),
        )
        .expect("Unable to read the auth data file.")
    }

    pub fn save(&self) {
        write(
            AUTH_CONFIG_FILE,
            toml::to_string_pretty(&self).expect("Unexpected error: unable serialize auth data."),
        )
        .expect("Unable to save auth data.");
    }

    pub fn expired(&self) -> bool {
        self.expires_at < current_unix_time()
    }

    pub fn update_from_expires_in(&mut self, expires_in: u128) {
        self.expires_at = current_unix_time() + expires_in;
    }
}
