//! Payouts API
//!
//! Payouts represent transfers of funds to your bank account.

use crate::{
    http::HttpClient,
    types::{List, ListParams, PayoutId, Timestamp},
    Result,
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

    pub async fn retrieve(&self, id: &PayoutId) -> Result<Payout> {
        self.http.get(&format!("/payouts/{}", id.as_str())).await
    }

    pub async fn list(&self, _params: ListParams) -> Result<List<Payout>> {
        self.http.get("/payouts").await
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutStatus {
    Pending,
    InTransit,
    Paid,
    Failed,
    Cancelled,
}
