//! Payments API
//!
//! Payments represent successful payment transactions.

use crate::{
    Result,
    http::HttpClient,
    types::{Currency, Metadata, PaymentId, PaymentIntentId, Timestamp},
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_refunded: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<Metadata>,
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<i64>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<i64>,
    pub payment_intent_id: PaymentIntentId,
    pub status: PaymentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded: Option<bool>,
    pub created_at: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
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
