//! Webhooks API
//!
//! Webhooks allow you to receive real-time notifications about events.

use crate::{
    Result,
    http::HttpClient,
    types::{List, ListParams, Timestamp, WebhookId, event::EventType},
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
            .put(&format!("/webhooks/{}", id.as_str()), &params)
            .await
    }

    pub async fn delete(&self, id: &WebhookId) -> Result<()> {
        self.http
            .delete(&format!("/webhooks/{}", id.as_str()))
            .await
    }

    pub async fn list(&self, params: WebhookListParams) -> Result<List<Webhook>> {
        self.http.get_with_params("/webhooks", &params).await
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
    pub events: Vec<EventType>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
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
    pub events: Vec<EventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<EventType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ListParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateWebhook {
    #[must_use]
    pub fn new(url: impl Into<String>, events: Vec<EventType>) -> Self {
        Self {
            url: url.into(),
            events,
            description: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl UpdateWebhook {
    #[must_use]
    pub fn new() -> Self {
        Self {
            url: None,
            events: None,
            description: None,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn events(mut self, events: Vec<EventType>) -> Self {
        self.events = Some(events);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}
