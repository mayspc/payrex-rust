//! Webhooks API
//!
//! Webhooks allow you to receive real-time notifications about events.

use crate::{
    Result,
    http::HttpClient,
    types::{List, ListParams, Timestamp, WebhookId},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct Webhooks {
    http: Arc<HttpClient>,
}

impl Webhooks {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn create(&self, params: CreateWebhook) -> Result<Webhook> {
        self.http.post("/webhooks", &params).await
    }

    pub async fn retrieve(&self, id: &WebhookId) -> Result<Webhook> {
        self.http.get(&format!("/webhooks/{}", id.as_str())).await
    }

    pub async fn update(&self, id: &WebhookId, params: UpdateWebhook) -> Result<Webhook> {
        self.http
            .patch(&format!("/webhooks/{}", id.as_str()), &params)
            .await
    }

    pub async fn delete(&self, id: &WebhookId) -> Result<()> {
        self.http
            .delete(&format!("/webhooks/{}", id.as_str()))
            .await
    }

    pub async fn list(&self, _params: ListParams) -> Result<List<Webhook>> {
        self.http.get("/webhooks").await
    }

    pub async fn enable(&self, id: &WebhookId) -> Result<Webhook> {
        self.http
            .post(&format!("/webhooks/{}/enable", id.as_str()), &())
            .await
    }

    pub async fn disable(&self, id: &WebhookId) -> Result<Webhook> {
        self.http
            .post(&format!("/webhooks/{}/disable", id.as_str()), &())
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Webhook {
    pub id: WebhookId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    pub status: WebhookStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub livemode: bool,
    pub url: String,
    pub events: Vec<String>,
    pub created_at: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhook {
    pub url: String,
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
