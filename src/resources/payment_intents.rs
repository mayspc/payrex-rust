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

    pub async fn capture(
        &self,
        id: &PaymentIntentId,
        params: CapturePaymentIntent,
    ) -> Result<PaymentIntent> {
        self.http
            .post(
                &format!("/payment_intents/{}/capture", id.as_str()),
                &params,
            )
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardOptions>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_type: Option<CaptureType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_bins: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_funding: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureType {
    Automatic,
    Manual,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NextAction {
    #[serde(rename = "type")]
    pub action_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
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
    pub last_payment_error: Option<PaymentError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    pub status: PaymentIntentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<NextAction>,
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
    pub payment_methods: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMethod {
    Automatic,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturePaymentIntent {
    pub amount: i64,
}

impl CapturePaymentIntent {
    #[must_use]
    pub const fn new(amount: i64) -> Self {
        Self { amount }
    }
}

impl CreatePaymentIntent {
    #[must_use]
    pub fn new(amount: i64, currency: Currency, payment_methods: Vec<String>) -> Self {
        Self {
            amount,
            currency,
            payment_methods,
            description: None,
            metadata: None,
            capture_method: None,
            payment_method_options: None,
            statement_descriptor: None,
            return_url: None,
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

    #[must_use]
    pub fn payment_method_options(mut self, options: PaymentMethodOptions) -> Self {
        self.payment_method_options = Some(options);
        self
    }

    #[must_use]
    pub fn statement_descriptor(mut self, descriptor: impl Into<String>) -> Self {
        self.statement_descriptor = Some(descriptor.into());
        self
    }

    #[must_use]
    pub fn return_url(mut self, url: impl Into<String>) -> Self {
        self.return_url = Some(url.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_payment_intent_builder() {
        let payment_methods = vec!["card".to_string(), "gcash".to_string()];
        let params = CreatePaymentIntent::new(10000, Currency::PHP, payment_methods.clone())
            .description("Test payment")
            .capture_method(CaptureMethod::Manual);

        assert_eq!(params.amount, 10000);
        assert_eq!(params.currency, Currency::PHP);
        assert_eq!(params.payment_methods, payment_methods);
        assert_eq!(params.description, Some("Test payment".to_string()));
        assert_eq!(params.capture_method, Some(CaptureMethod::Manual));
    }

    #[test]
    fn test_create_payment_intent_with_all_options() {
        let payment_methods = vec!["card".to_string()];
        let mut metadata = Metadata::new();
        metadata.insert("order_id", "12345");

        let card_options = CardOptions {
            capture_type: Some(CaptureType::Manual),
            allowed_bins: Some(vec!["123456".to_string()]),
            allowed_funding: Some(vec!["credit".to_string()]),
        };

        let payment_method_options = PaymentMethodOptions {
            card: Some(card_options),
        };

        let params = CreatePaymentIntent::new(10000, Currency::PHP, payment_methods)
            .description("Test payment")
            .metadata(metadata.clone())
            .capture_method(CaptureMethod::Manual)
            .payment_method_options(payment_method_options.clone())
            .statement_descriptor("TEST MERCHANT")
            .return_url("https://example.com/return");

        assert_eq!(params.amount, 10000);
        assert_eq!(params.description, Some("Test payment".to_string()));
        assert_eq!(params.metadata, Some(metadata));
        assert_eq!(params.capture_method, Some(CaptureMethod::Manual));
        assert!(params.payment_method_options.is_some());
        assert_eq!(
            params.statement_descriptor,
            Some("TEST MERCHANT".to_string())
        );
        assert_eq!(
            params.return_url,
            Some("https://example.com/return".to_string())
        );
    }

    #[test]
    fn test_capture_payment_intent() {
        let params = CapturePaymentIntent::new(5000);
        assert_eq!(params.amount, 5000);
    }

    #[test]
    fn test_payment_intent_status_serialization() {
        use serde_json;

        let status = PaymentIntentStatus::RequiresPaymentMethod;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"requires_payment_method\"");

        let status = PaymentIntentStatus::Succeeded;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"succeeded\"");
    }

    #[test]
    fn test_capture_type_serialization() {
        use serde_json;

        let capture_type = CaptureType::Automatic;
        let json = serde_json::to_string(&capture_type).unwrap();
        assert_eq!(json, "\"automatic\"");

        let capture_type = CaptureType::Manual;
        let json = serde_json::to_string(&capture_type).unwrap();
        assert_eq!(json, "\"manual\"");
    }
}
