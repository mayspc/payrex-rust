//! Events API
//!
//! Events represent changes to resources in your account.

use crate::{
    Result,
    http::HttpClient,
    types::{EventId, List, ListParams, Timestamp},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct Events {
    http: Arc<HttpClient>,
}

impl Events {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn retrieve(&self, id: &EventId) -> Result<Event> {
        self.http.get(&format!("/events/{}", id.as_str())).await
    }

    pub async fn list(&self, _params: ListParams) -> Result<List<Event>> {
        self.http.get("/events").await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub data: Value,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_webhooks: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_attributes: Option<Value>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
