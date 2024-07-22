use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionLevel {
    pub created_at: u64,
    pub currency_prices: HashMap<String, f32>,
    pub deleted: bool,
    pub id: u64,
    pub is_archived: bool,
    pub name: String,
    pub owner_id: u64,
    pub price: f32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BasicSubscriber {
    pub avatar_url: String,
    pub email: String,
    pub has_avatar: bool,
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subscriber {
    pub can_write: bool,
    pub is_black_listed: bool,
    pub level: SubscriptionLevel,
    pub on_time: u128,
    pub payments: f32,
    pub price: f32,
    pub subscribed: bool,

    #[serde(flatten)]
    pub basic_info: BasicSubscriber,
}

impl Subscriber {
    pub fn is_paid(&self) -> bool {
        self.subscribed && self.price > 0.
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct SortBy(String);

impl Default for SortBy {
    fn default() -> Self {
        Self("on_time".to_string())
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Order(String);

impl Default for Order {
    fn default() -> Self {
        Self("gt".to_string())
    }
}

#[derive(Debug, Clone)]
pub struct SubscribersVec(Vec<u64>);

impl From<Vec<u64>> for SubscribersVec {
    fn from(value: Vec<u64>) -> Self {
        Self(value)
    }
}

impl Serialize for SubscribersVec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(
            self.0
                .iter()
                .map(|&id| id.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .as_str(),
        )
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct SubscribersRequest {
    #[serde(default)]
    pub sort_by: SortBy,
    pub limit: u32,
    pub offset: Option<u32>,
    #[serde(default)]
    pub order: Order,
    pub user_ids: SubscribersVec,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscribersResponse {
    pub data: Vec<Subscriber>,
    pub limit: u32,
    pub offset: u32,
    pub total: u32,
}

#[derive(Serialize, Debug, Clone)]
pub struct SearchRequest {
    pub chunk: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub data: Vec<BasicSubscriber>,
}
