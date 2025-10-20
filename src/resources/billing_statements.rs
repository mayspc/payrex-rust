//! Billing Statements API
//!
//! Billing Statements allow you to create and send invoices to customers.

use crate::{
    Result,
    http::HttpClient,
    resources::{
        billing_statement_line_items::BillingStatementLineItem, customers::Customer,
        payment_intents::PaymentIntent,
    },
    types::{
        BillingStatementId, Currency, CustomerId, List, ListParams, Metadata, PaymentMethod,
        Timestamp,
    },
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
/// Billing statements are one-time payment links that contain customer information, the due date,
/// and an itemized list of your business's products or services.
pub struct BillingStatements {
    http: Arc<HttpClient>,
}

impl BillingStatements {
    #[must_use]
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Creates a billing statement resource.
    ///
    /// Endpoint: `POST /billing_statements`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/create)
    pub async fn create(&self, params: CreateBillingStatement) -> Result<BillingStatement> {
        self.http.post("/billing_statements", &params).await
    }

    /// Retrieves a billing statement resource.
    ///
    /// Endpoint: `GET /billing_statements/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/retrieve)
    pub async fn retrieve(&self, id: &BillingStatementId) -> Result<BillingStatement> {
        self.http
            .get(&format!("/billing_statements/{}", id.as_str()))
            .await
    }

    /// Updates a billing statement resource.
    ///
    /// Endpoint: `PUT /billing_statements/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/update)
    pub async fn update(
        &self,
        id: &BillingStatementId,
        params: UpdateBillingStatement,
    ) -> Result<BillingStatement> {
        self.http
            .put(&format!("/billing_statements/{}", id.as_str()), &params)
            .await
    }

    /// Deletes a billing statement resource.
    ///
    /// Endpoint: `DELETE /billing_statements/:id`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/delete)
    pub async fn delete(&self, id: &BillingStatementId) -> Result<()> {
        self.http
            .delete(&format!("/billing_statements/{}", id.as_str()))
            .await
    }

    /// List billing statement resources.
    ///
    /// Endpoint: `GET /billing_statements`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/list)
    pub async fn list(&self, params: Option<ListParams>) -> Result<List<BillingStatement>> {
        self.http
            .get_with_params("/billing_statements", &params)
            .await
    }

    /// Finalizes a billing statement resource.
    ///
    /// Endpoint: `POST /billing_statements/:id/finalize`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/finalize)
    pub async fn finalize(&self, id: &BillingStatementId) -> Result<BillingStatement> {
        self.http
            .post(
                &format!("/billing_statements/{}/finalize", id.as_str()),
                &(),
            )
            .await
    }

    /// Send a billing statement via e-mail.
    ///
    /// Endpoint: `POST /billing_statements/:id/send`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/send)
    pub async fn send(&self, id: &BillingStatementId) -> Result<BillingStatement> {
        self.http
            .post(&format!("/billing_statements/{}/send", id.as_str()), &())
            .await
    }

    /// Voids a billing statement resource.
    ///
    /// Endpoint: `POST /billing_statements/:id/void`
    ///
    /// [API Reference](https://docs.payrexhq.com/docs/api/billing_statements/void)
    pub async fn void(&self, id: &BillingStatementId) -> Result<BillingStatement> {
        self.http
            .post(&format!("/billing_statements/{}/void", id.as_str()), &())
            .await
    }
}

/// Billing Statement Resource.
///
/// [Learn more about it here](https://docs.payrexhq.com/docs/api/billing_statements)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingStatement {
    /// The ID of a customer resource. To learn more about the customer resource, you can refer
    /// [here](https://docs.payrexhq.com/docs/api/customers).
    pub id: BillingStatementId,

    /// The final amount collected by the `BillingStatement` is a positive integer representing the
    /// amount your customer will pay in the smallest currency unit, cents. If the customer pays ₱
    /// 120.50, the amount of the `BillingStatement` should be 12050.
    ///
    /// The `BillingStatement` amount is derived from the sum of all `line_items.quantity *
    /// line_items.unit_price`.
    ///
    /// The minimum amount is ₱ 20 (2000 in cents), and the maximum amount is ₱ 59,999,999.99
    /// (5999999999 in cents).
    pub amount: i64,

    /// A three-letter ISO currency code, in uppercase. As of the moment, we only support PHP.
    ///
    /// This value is derived from the currency of the associated customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details_collection: Option<String>,

    /// A three-letter ISO currency code, in uppercase. As of the moment, we only support PHP.
    ///
    /// This value is derived from the currency of the associated customer.
    pub currency: Currency,

    /// The ID of a customer resource. To learn more about the customer resource, you can refer
    /// [here](https://docs.payrexhq.com/docs/api/customers).
    pub customer_id: CustomerId,

    /// An arbitrary string attached to the billing statement and copied over to its payment
    /// intent. This is a useful reference when viewing the payment resources associated with the
    /// billing statement from the PayRex Dashboard.
    ///
    /// If the description is not modified, the default value is "Payment for Billing Statement
    /// <billing statement number>"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The time when the billing statement is expected to be paid. If the `due_at` is already past,
    /// your customer can still pay the billing statement if the status is open.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized_at: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_merchant_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_number: Option<String>,

    /// The URL that your customer will access to pay the billing statement.
    ///
    /// This is only visible if the billing statement's status [`BillingStatementStatus`] is `open`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_statement_url: Option<String>,

    /// This attribute holds the billing statement's list of line items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<BillingStatementLineItem>>,

    /// The value is `true` if the resource's mode is live, and the value is `false` if the resource is
    /// in test mode.
    pub livemode: bool,

    /// Set of key-value pairs attached to the billing statement. This is useful for storing
    /// additional information about the billing statement.
    ///
    /// The latest value of the billing statement's metadata will be copied to its payment intent
    /// once the billing statement is finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The [PaymentIntent](https://docs.payrexhq.com/docs/api/payment_intents) resource created for the [`BillingStatement`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<PaymentIntent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    pub status: BillingStatementStatus,
    pub payment_settings: PaymentSettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,

    /// The time the resource was created and measured in seconds since the Unix epoch.
    pub created_at: Timestamp,

    /// The time the resource was updated and measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentSettings {
    pub payment_methods: Vec<PaymentMethod>,
}

/// The latest status of the [`BillingStatement`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingStatementStatus {
    /// The latest status is draft.
    Draft,

    /// The latest status is open.
    Open,

    /// The latest status is paid.
    Paid,

    /// The latest status is uncollectible.
    Void,

    /// The latest status is uncollectible.
    Uncollectible,
}

/// Query parameters when creating a billing statement.
///
/// [Reference](https://docs.payrexhq.com/docs/api/billing_statements/create#parameters)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBillingStatement {
    /// The ID of a customer resource. To learn more about the customer resource, you can refer
    /// [here](https://docs.payrexhq.com/docs/api/customers).
    pub customer_id: CustomerId,

    /// A three-letter ISO currency code, in uppercase. As of the moment, we only support PHP.
    ///
    /// This value is derived from the currency of the associated customer.
    pub currency: Currency,

    pub payment_settings: PaymentSettings,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details_collection: Option<String>,

    /// An arbitrary string attached to the billing statement and copied over to its payment
    /// intent. This is a useful reference when viewing the payment resources associated with the
    /// billing statement from the PayRex Dashboard.
    ///
    /// If the description is not modified, the default value is "Payment for Billing Statement
    /// <billing statement number>"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Set of key-value pairs attached to the billing statement. This is useful for storing
    /// additional information about the billing statement.
    ///
    /// The latest value of the billing statement's metadata will be copied to its payment intent
    /// once the billing statement is finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBillingStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<PaymentSettings>,

    /// An arbitrary string attached to the billing statement and copied over to its payment
    /// intent. This is a useful reference when viewing the payment resources associated with the
    /// billing statement from the PayRex Dashboard.
    ///
    /// If the description is not modified, the default value is "Payment for Billing Statement
    /// <billing statement number>"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Set of key-value pairs attached to the billing statement. This is useful for storing
    /// additional information about the billing statement.
    ///
    /// The latest value of the billing statement's metadata will be copied to its payment intent
    /// once the billing statement is finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl CreateBillingStatement {
    #[must_use]
    pub fn new(
        customer_id: CustomerId,
        currency: Currency,
        payment_settings: PaymentSettings,
    ) -> Self {
        Self {
            customer_id,
            currency,
            payment_settings,
            billing_details_collection: None,
            description: None,
            metadata: None,
        }
    }

    pub fn billing_details_collection(mut self, collection: impl Into<String>) -> Self {
        self.billing_details_collection = Some(collection.into());
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

impl UpdateBillingStatement {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn customer_id(mut self, id: CustomerId) -> Self {
        self.customer_id = Some(id);
        self
    }

    pub fn payment_settings(mut self, settings: PaymentSettings) -> Self {
        self.payment_settings = Some(settings);
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
