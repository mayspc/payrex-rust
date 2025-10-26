//! Checkout Sessions API
//!
//! Checkout Sessions create a hosted payment page for collecting payment.

use crate::{
    Result,
    http::HttpClient,
    resources::payment_intents::PaymentIntent,
    types::{
        CheckoutSessionId, CheckoutSessionLineItemId, Currency, Metadata, PaymentMethod,
        PaymentMethodOptions, Timestamp,
    },
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
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details_collection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub status: CheckoutSessionStatus,
    pub currency: Currency,
    pub line_items: Vec<CheckoutSessionLineItem>,
    pub livemode: bool,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<PaymentIntent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<PaymentMethod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionStatus {
    Active,
    Completed,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckoutSessionLineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<CheckoutSessionLineItemId>,
    pub name: String,
    pub amount: u64,
    pub quantity: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckoutSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reference_id: Option<String>,
    pub currency: Currency,
    pub line_items: Vec<CheckoutSessionLineItem>,
    pub success_url: String,
    pub cancel_url: String,
    pub payment_methods: Vec<PaymentMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details_collection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl CreateCheckoutSession {
    #[must_use]
    pub fn new(
        currency: Currency,
        line_items: Vec<CheckoutSessionLineItem>,
        success_url: impl Into<String>,
        cancel_url: impl Into<String>,
        payment_methods: Vec<PaymentMethod>,
    ) -> Self {
        Self {
            customer_reference_id: None,
            currency,
            line_items,
            success_url: success_url.into(),
            cancel_url: cancel_url.into(),
            payment_methods,
            payment_method_options: None,
            expires_at: None,
            billing_details_collection: None,
            submit_type: None,
            description: None,
            metadata: None,
        }
    }

    pub fn customer_reference_id(mut self, id: impl Into<String>) -> Self {
        self.customer_reference_id = Some(id.into());
        self
    }

    pub fn expires_at(mut self, timestamp: Timestamp) -> Self {
        self.expires_at = Some(timestamp);
        self
    }

    pub fn payment_method_options(mut self, options: PaymentMethodOptions) -> Self {
        self.payment_method_options = Some(options);
        self
    }

    pub fn billing_details_collection(mut self, collection: impl Into<String>) -> Self {
        self.billing_details_collection = Some(collection.into());
        self
    }

    pub fn submit_type(mut self, submit_type: impl Into<String>) -> Self {
        self.submit_type = Some(submit_type.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl CheckoutSessionLineItem {
    #[must_use]
    pub fn new(name: impl Into<String>, amount: u64, quantity: u64) -> Self {
        Self {
            id: None,
            name: name.into(),
            amount,
            quantity,
            description: None,
            image: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }
}
