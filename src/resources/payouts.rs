//! Payouts API
//!
//! Payouts represent transfers of funds to your bank account.

use crate::{
    Result,
    http::HttpClient,
    types::{List, ListParams, PayoutId, PayoutTransactionId, Timestamp},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct Payouts {
    http: Arc<HttpClient>,
}

impl Payouts {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn list_transactions(
        &self,
        id: &PayoutId,
        params: Option<ListParams>,
    ) -> Result<List<PayoutTransaction>> {
        self.http
            .get_with_params(&format!("/payouts/{}/transactions", id.as_str()), &params)
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Payout {
    pub id: PayoutId,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<i64>,
    pub status: PayoutStatus,
    pub created_at: Timestamp,
    pub updated_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutStatus {
    Pending,
    InTransit,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutTransactionType {
    Payment,
    Refund,
    Adjustment,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayoutTransaction {
    pub id: PayoutTransactionId,
    pub amount: i32,
    pub net_amount: i32,
    // TODO: identify the type of resource id based on `transaction_type`
    pub transaction_id: PayoutTransactionId,
    pub transaction_type: PayoutTransactionType,
    pub created_at: Timestamp,
    pub updated_at: Option<Timestamp>,
}
