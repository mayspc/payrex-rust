//! Customers API
//!
//! Customers represent your business's customers and allow you to track
//! multiple payments and billing information.

use crate::{
    Result,
    http::HttpClient,
    types::{Currency, CustomerId, List, ListParams, Metadata, Timestamp},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct Customers {
    http: Arc<HttpClient>,
}

impl Customers {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub async fn create(&self, params: CreateCustomer) -> Result<Customer> {
        self.http.post("/customers", &params).await
    }

    pub async fn retrieve(&self, id: &CustomerId) -> Result<Customer> {
        self.http.get(&format!("/customers/{}", id.as_str())).await
    }

    pub async fn update(&self, id: &CustomerId, params: UpdateCustomer) -> Result<Customer> {
        self.http
            .patch(&format!("/customers/{}", id.as_str()), &params)
            .await
    }

    pub async fn delete(&self, id: &CustomerId) -> Result<()> {
        self.http
            .delete(&format!("/customers/{}", id.as_str()))
            .await
    }

    pub async fn list(&self, _params: ListParams) -> Result<List<Customer>> {
        self.http.get("/customers").await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Customer {
    pub id: CustomerId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_statement_sequence_number: Option<i64>,
    pub created_at: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCustomer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateCustomer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
