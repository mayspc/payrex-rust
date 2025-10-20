//! Billing Statement Line Items API
//!
//! Billing Statement Line Items allows you to create, update, and delete statement line items.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    Result,
    http::HttpClient,
    types::{BillingStatementId, BillingStatementLineItemId, Timestamp},
};

#[derive(Clone)]
pub struct BillingStatementLineItems {
    http: Arc<HttpClient>,
}

impl BillingStatementLineItems {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn create(
        &self,
        params: CreateBillingStatementLineItem,
    ) -> Result<BillingStatementLineItem> {
        self.http
            .post("/billing_statement_line_items", &params)
            .await
    }

    pub async fn update(
        &self,
        id: BillingStatementLineItemId,
        params: UpdateBillingStatementLineItem,
    ) -> Result<BillingStatementLineItem> {
        self.http
            .put(
                &format!("/billing_statement_line_items/{}", id.as_str()),
                &params,
            )
            .await
    }

    pub async fn delete(&self, id: &BillingStatementLineItemId) -> Result<()> {
        self.http
            .delete(&format!("/billing_statement_line_items/{}", id.as_str()))
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingStatementLineItem {
    pub id: BillingStatementLineItemId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub unit_price: u64,
    pub quantity: u64,
    pub billing_statement_id: BillingStatementId,
    pub livemode: bool,
    pub created_at: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateBillingStatementLineItem {
    pub billing_statement_id: BillingStatementId,
    pub description: String,
    pub unit_price: u64,
    pub quantity: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpdateBillingStatementLineItem {
    pub description: Option<String>,
    pub unit_price: Option<u64>,
    pub quantity: Option<u64>,
}

impl CreateBillingStatementLineItem {
    #[must_use]
    pub fn new(
        billing_statement_id: BillingStatementId,
        description: impl Into<String>,
        unit_price: u64,
        quantity: u64,
    ) -> Self {
        Self {
            billing_statement_id,
            description: description.into(),
            unit_price,
            quantity,
        }
    }
}

impl UpdateBillingStatementLineItem {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn unit_price(mut self, price: u64) -> Self {
        self.unit_price = Some(price);
        self
    }

    pub fn quantity(mut self, quantity: u64) -> Self {
        self.quantity = Some(quantity);
        self
    }
}
