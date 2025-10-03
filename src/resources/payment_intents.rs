//! Payment Intents API
//!
//! Payment Intents represent an intent to collect payment from a customer.
//! They track the lifecycle of a payment from creation through completion.

use crate::{
    Result,
    http::HttpClient,
    types::{Currency, Metadata, PaymentIntentId, Timestamp},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct PaymentIntents {
    http: Arc<HttpClient>,
}

impl PaymentIntents {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn create(&self, params: CreatePaymentIntent) -> Result<PaymentIntent> {
        self.http.post("/payment_intents", &params).await
    }

    pub async fn retrieve(&self, id: &PaymentIntentId) -> Result<PaymentIntent> {
        self.http
            .get(&format!("/payment_intents/{}", id.as_str()))
            .await
    }

    pub async fn cancel(&self, id: &PaymentIntentId) -> Result<PaymentIntent> {
        self.http
            .post(&format!("/payment_intents/{}/cancel", id.as_str()), &())
            .await
    }

    pub async fn capture(&self, id: &PaymentIntentId) -> Result<PaymentIntent> {
        self.http
            .post(&format!("/payment_intents/{}/capture", id.as_str()), &())
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentIntent {
    pub id: PaymentIntentId,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_received: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_capturable: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_payment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_error: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    pub status: PaymentIntentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_before_at: Option<Timestamp>,
    pub created_at: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentStatus {
    RequiresPaymentMethod,
    RequiresConfirmation,
    RequiresAction,
    Processing,
    RequiresCapture,
    Cancelled,
    Succeeded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentIntent {
    pub amount: i64,
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CaptureMethod>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMethod {
    Automatic,
    Manual,
}

impl CreatePaymentIntent {
    #[must_use]
    pub const fn new(amount: i64, currency: Currency) -> Self {
        Self {
            amount,
            currency,
            description: None,
            metadata: None,
            capture_method: None,
        }
    }

    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    #[must_use]
    pub const fn capture_method(mut self, method: CaptureMethod) -> Self {
        self.capture_method = Some(method);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_payment_intent_builder() {
        let params = CreatePaymentIntent::new(10000, Currency::PHP)
            .description("Test payment")
            .capture_method(CaptureMethod::Manual);

        assert_eq!(params.amount, 10000);
        assert_eq!(params.currency, Currency::PHP);
        assert_eq!(params.description, Some("Test payment".to_string()));
        assert_eq!(params.capture_method, Some(CaptureMethod::Manual));
    }
}
