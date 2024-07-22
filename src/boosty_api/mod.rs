use std::{fmt::Display, sync::Arc};

use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use tokio::sync::RwLock;

use self::types::{auth::*, subscribers::*};

pub mod auth;
pub mod types;

pub type RequestResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Debug)]
struct BaseUrl(String);

impl Default for BaseUrl {
    fn default() -> Self {
        Self("https://api.boosty.to".to_string())
    }
}

impl Display for BaseUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct BoostyClient {
    base_url: BaseUrl,
    auth: Arc<RwLock<auth::AuthData>>,
    client: reqwest::Client,
}

impl BoostyClient {
    async fn prepare_request(&self, request: RequestBuilder) -> RequestBuilder {
        request.bearer_auth(self.auth.read().await)
    }

    async fn send_request_json_no_auth_check<R>(&self, request: RequestBuilder) -> RequestResult<R>
    where
        R: DeserializeOwned,
    {
        Ok(request
            .send()
            .await
            .unwrap()
            .error_for_status()?
            .json::<R>()
            .await?)
    }

    async fn send_request_json<R>(&self, request: RequestBuilder) -> RequestResult<R>
    where
        R: DeserializeOwned,
    {
        self.refresh_auth_if_expired().await?;
        self.send_request_json_no_auth_check::<R>(request).await
    }

    fn full_url(&self, url: impl ToString) -> String {
        format!("{}{}", self.base_url, url.to_string())
    }

    pub async fn refresh_auth_if_expired(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let auth_data = self.auth.read().await.clone();

        if !auth_data.expired() {
            return Ok(());
        }

        let result = self
            .send_request_json_no_auth_check::<RefreshAuthDataResponse>(
                self.prepare_request(self.client.post(self.full_url("/oauth/token/")))
                    .await
                    .form(&RefreshAuthDataRequest {
                        device_id: auth_data.device_id,
                        device_os: auth_data.device_os,
                        grant_type: auth_data.grant_type,
                        refresh_token: auth_data.refresh_token,
                    }),
            )
            .await?;

        let mut auth_data = self.auth.write().await;

        auth_data.access_token = result.access_token;
        auth_data.refresh_token = result.refresh_token;
        auth_data.update_from_expires_in(result.expires_in);

        auth_data.save();

        Ok(())
    }

    pub async fn subscribers(
        &self,
        data: &SubscribersRequest,
    ) -> RequestResult<SubscribersResponse> {
        self.send_request_json::<SubscribersResponse>(
            self.prepare_request(
                self.client
                    .get(self.full_url("/v1/blog/hedgehoginc/subscribers")),
            )
            .await
            .query(data),
        )
        .await
    }

    pub async fn search(&self, data: &SearchRequest) -> RequestResult<SearchResponse> {
        self.send_request_json::<SearchResponse>(
            self.prepare_request(
                self.client
                    .get(self.full_url("/v1/blog/stat/hedgehoginc/search")),
            )
            .await
            .query(data),
        )
        .await
    }
}

pub struct BoostyClientBuilder {
    base_url: BaseUrl,
    auth: auth::AuthData,
}

impl BoostyClientBuilder {
    pub fn new(auth: auth::AuthData) -> Self {
        Self {
            base_url: BaseUrl::default(),
            auth,
        }
    }

    pub fn with_custom_base_url(mut self, new_url: String) -> Self {
        self.base_url = BaseUrl(new_url);

        self
    }

    pub fn build(&self) -> BoostyClient {
        BoostyClient {
            base_url: self.base_url.clone(),
            auth: Arc::new(RwLock::new(self.auth.clone())),
            client: reqwest::Client::new(),
        }
    }
}
