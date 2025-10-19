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

    pub async fn list(&self, _params: Option<CustomerListParams>) -> Result<List<Customer>> {
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
    pub next_billing_statement_sequence_number: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCustomer {
    pub currency: Currency,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_statement_sequence_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateCustomer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_statement_sequence_number: Option<String>,
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
pub struct CustomerListParams {
    #[serde(flatten)]
    pub list_params: ListParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl CreateCustomer {
    #[must_use]
    pub fn new(currency: Currency, email: String, name: String) -> Self {
        Self {
            currency,
            email,
            name,
            ..Default::default()
        }
    }

    pub fn billing_statement_prefix(mut self, billing_statement_prefix: impl Into<String>) -> Self {
        self.billing_statement_prefix = Some(billing_statement_prefix.into());
        self
    }

    pub fn next_billing_statement_sequence_number(
        mut self,
        sequence_number: impl Into<String>,
    ) -> Self {
        self.next_billing_statement_sequence_number = Some(sequence_number.into());
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

// TODO: maybe consider `derive_builder` crate
impl UpdateCustomer {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn billing_statement_prefix(mut self, billing_statement_prefix: impl Into<String>) -> Self {
        self.billing_statement_prefix = Some(billing_statement_prefix.into());
        self
    }

    pub fn next_billing_statement_sequence_number(
        mut self,
        sequence_number: impl Into<String>,
    ) -> Self {
        self.next_billing_statement_sequence_number = Some(sequence_number.into());
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl CustomerListParams {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}
