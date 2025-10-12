//! Payments API
//!
//! Payments represent successful payment transactions.

use crate::{
    Result,
    http::HttpClient,
    types::{Currency, Metadata, PaymentId, PaymentIntentId, PaymentMethod, Timestamp},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct Payments {
    http: Arc<HttpClient>,
}

impl Payments {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn retrieve(&self, id: &PaymentId) -> Result<Payment> {
        self.http.get(&format!("/payments/{}", id.as_str())).await
    }

    pub async fn update(&self, id: &PaymentId, params: UpdatePayment) -> Result<Payment> {
        self.http
            .patch(&format!("/payments/{}", id.as_str()), &params)
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Payment {
    pub id: PaymentId,
    pub amount: i64,
    pub amount_refunded: i64,
    pub billing: Billing,
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub fee: i64,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    pub net_amount: i64,
    pub payment_intent_id: PaymentIntentId,
    pub status: PaymentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Metadata>, // TODO: Add Customer type
    pub payment_method: PaymentMethodTypes,
    pub refunded: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Billing {
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    pub address: Address,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentMethodTypes {
    #[serde(rename = "type")]
    pub _type: PaymentMethod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Succeeded,
    Failed,
    Pending,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePayment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl UpdatePayment {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
