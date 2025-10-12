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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_payment_builder() {
        let mut metadata = Metadata::new();
        metadata.insert("order_id".to_string(), "12345".to_string());

        let params = UpdatePayment::new()
            .description("Test payment")
            .metadata(metadata.clone());

        assert_eq!(params.description, Some("Test payment".to_string()));
        assert_eq!(params.metadata, Some(metadata));
    }

    #[test]
    fn test_payment_status_serialization() {
        let status = PaymentStatus::Succeeded;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"succeeded\"");

        let status = PaymentStatus::Failed;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"failed\"");
    }

    #[test]
    fn test_address_serialization() {
        let address = Address {
            line1: "BGC".to_string(),
            line2: Some("Apt 4B".to_string()),
            city: "Taguig".to_string(),
            state: "NCR".to_string(),
            postal_code: "1635".to_string(),
            country: "PH".to_string(),
        };

        let serialized = serde_json::to_string(&address).unwrap();
        let expected = r#"{"line1":"BGC","line2":"Apt 4B","city":"Taguig","state":"NCR","postal_code":"1635","country":"PH"}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_billing_serialization() {
        let address = Address {
            line1: "BGC".to_string(),
            line2: Some("Apt 4B".to_string()),
            city: "Taguig".to_string(),
            state: "NCR".to_string(),
            postal_code: "1635".to_string(),
            country: "PH".to_string(),
        };

        let billing = Billing {
            name: "John Doe".to_string(),
            email: "johndoe@gmail.com".to_string(),
            phone: Some("1234567890".to_string()),
            address,
        };

        let serialized = serde_json::to_string(&billing).unwrap();
        let expected = r#"{"name":"John Doe","email":"johndoe@gmail.com","phone":"1234567890","address":{"line1":"BGC","line2":"Apt 4B","city":"Taguig","state":"NCR","postal_code":"1635","country":"PH"}}"#;

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_payment_method_types_serialization() {
        let payment_method = PaymentMethodTypes {
            _type: PaymentMethod::Card,
        };

        let serialized = serde_json::to_string(&payment_method).unwrap();
        let expected = r#"{"type":"card"}"#;

        assert_eq!(serialized, expected);
    }
}
