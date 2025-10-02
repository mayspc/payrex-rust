//! Billing Statements API
//!
//! Billing Statements allow you to create and send invoices to customers.

use crate::{
    http::HttpClient,
    types::{BillingStatementId, Currency, CustomerId, List, ListParams, Metadata, Timestamp},
    Result,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct BillingStatements {
    http: Arc<HttpClient>,
}

impl BillingStatements {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn create(&self, params: CreateBillingStatement) -> Result<BillingStatement> {
        self.http.post("/billing_statements", &params).await
    }

    pub async fn retrieve(&self, id: &BillingStatementId) -> Result<BillingStatement> {
        self.http
            .get(&format!("/billing_statements/{}", id.as_str()))
            .await
    }

    pub async fn update(
        &self,
        id: &BillingStatementId,
        params: UpdateBillingStatement,
    ) -> Result<BillingStatement> {
        self.http
            .patch(&format!("/billing_statements/{}", id.as_str()), &params)
            .await
    }

    pub async fn delete(&self, id: &BillingStatementId) -> Result<()> {
        self.http
            .delete(&format!("/billing_statements/{}", id.as_str()))
            .await
    }

    pub async fn list(&self, _params: ListParams) -> Result<List<BillingStatement>> {
        self.http.get("/billing_statements").await
    }

    pub async fn finalize(&self, id: &BillingStatementId) -> Result<BillingStatement> {
        self.http
            .post(
                &format!("/billing_statements/{}/finalize", id.as_str()),
                &(),
            )
            .await
    }

    pub async fn send(&self, id: &BillingStatementId) -> Result<BillingStatement> {
        self.http
            .post(&format!("/billing_statements/{}/send", id.as_str()), &())
            .await
    }

    pub async fn void(&self, id: &BillingStatementId) -> Result<BillingStatement> {
        self.http
            .post(&format!("/billing_statements/{}/void", id.as_str()), &())
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingStatement {
    pub id: BillingStatementId,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details_collection: Option<String>,
    pub currency: Currency,
    pub customer_id: CustomerId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_merchant_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<Metadata>>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    pub status: BillingStatementStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Metadata>,
    pub created_at: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingStatementStatus {
    Draft,
    Open,
    Paid,
    Void,
    Uncollectible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBillingStatement {
    pub customer: CustomerId,
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBillingStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
