//! Refunds API
//!
//! Refunds allow you to return money to a customer.

use crate::{
    Result,
    http::HttpClient,
    types::{Currency, Metadata, PaymentId, RefundId, Timestamp},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct Refunds {
    http: Arc<HttpClient>,
}

impl Refunds {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn create(&self, params: CreateRefund) -> Result<Refund> {
        self.http.post("/refunds", &params).await
    }

    pub async fn update(&self, id: &RefundId, params: UpdateRefund) -> Result<Refund> {
        self.http
            .put(&format!("/refunds/{}", id.as_str()), &params)
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Refund {
    pub id: RefundId,
    pub amount: i64,
    pub currency: Currency,
    pub livemode: bool,
    pub status: RefundStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub reason: RefundReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    pub payment_id: PaymentId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    Pending,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundReason {
    Fraudulent,
    RequestedByCustomer,
    ProductOutOfStock,
    ProductWasDamaged,
    ServiceNotProvided,
    ServiceMisaligned,
    WrongProductReceived,
    Others,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRefund {
    pub payment_id: PaymentId,
    pub amount: i64,
    pub currency: Currency,
    pub reason: RefundReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateRefund {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl CreateRefund {
    #[must_use]
    pub fn new(
        payment_id: PaymentId,
        amount: i64,
        currency: Currency,
        reason: RefundReason,
    ) -> Self {
        Self {
            payment_id,
            amount,
            currency,
            reason,
            metadata: None,
            remarks: None,
            description: None,
        }
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn remarks(mut self, remarks: impl Into<String>) -> Self {
        self.remarks = Some(remarks.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}
