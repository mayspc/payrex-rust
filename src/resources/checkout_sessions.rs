//! Checkout Sessions API
//!
//! Checkout Sessions create a hosted payment page for collecting payment.

use crate::{
    Result,
    http::HttpClient,
    types::{CheckoutSessionId, Currency, Metadata, Timestamp},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct CheckoutSessions {
    http: Arc<HttpClient>,
}

impl CheckoutSessions {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn create(&self, params: CreateCheckoutSession) -> Result<CheckoutSession> {
        self.http.post("/checkout_sessions", &params).await
    }

    pub async fn retrieve(&self, id: &CheckoutSessionId) -> Result<CheckoutSession> {
        self.http
            .get(&format!("/checkout_sessions/{}", id.as_str()))
            .await
    }

    pub async fn expire(&self, id: &CheckoutSessionId) -> Result<CheckoutSession> {
        self.http
            .post(&format!("/checkout_sessions/{}/expire", id.as_str()), &())
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckoutSession {
    pub id: CheckoutSessionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details_collection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub status: CheckoutSessionStatus,
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<Metadata>>,
    pub livemode: bool,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,
    pub created_at: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionStatus {
    Open,
    Complete,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckoutSession {
    pub amount: i64,
    pub currency: Currency,
    pub success_url: String,
    pub cancel_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
