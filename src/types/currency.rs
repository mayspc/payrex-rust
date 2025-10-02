//! Currency types for the PayRex SDK.
//!
//! PayRex primarily operates in Philippine Peso (PHP) but may support other currencies.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Currency {
    #[serde(rename = "php")]
    PHP,
    #[serde(rename = "usd")]
    USD,
}

impl Currency {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PHP => "php",
            Self::USD => "usd",
        }
    }

    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::PHP => "₱",
            Self::USD => "$",
        }
    }

    #[must_use]
    pub const fn decimal_places(self) -> u8 {
        match self {
            Self::PHP => 2,
            Self::USD => 2,
        }
    }

    /// Format an amount in the smallest currency unit (e.g., centavos for PHP).
    ///
    /// # Examples
    ///
    /// ```
    /// use payrex::types::Currency;
    ///
    /// let formatted = Currency::PHP.format_amount(10050);
    /// assert_eq!(formatted, "₱100.50");
    /// ```
    #[must_use]
    pub fn format_amount(self, amount: i64) -> String {
        let decimal_places = self.decimal_places();
        let divisor = 10_i64.pow(u32::from(decimal_places));
        let major = amount / divisor;
        let minor = (amount % divisor).abs();

        format!(
            "{}{}.{:0width$}",
            self.symbol(),
            major,
            minor,
            width = decimal_places as usize
        )
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Currency {
    fn default() -> Self {
        Self::PHP
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_as_str() {
        assert_eq!(Currency::PHP.as_str(), "php");
        assert_eq!(Currency::USD.as_str(), "usd");
    }

    #[test]
    fn test_currency_symbol() {
        assert_eq!(Currency::PHP.symbol(), "₱");
        assert_eq!(Currency::USD.symbol(), "$");
    }

    #[test]
    fn test_currency_decimal_places() {
        assert_eq!(Currency::PHP.decimal_places(), 2);
        assert_eq!(Currency::USD.decimal_places(), 2);
    }

    #[test]
    fn test_format_amount() {
        assert_eq!(Currency::PHP.format_amount(10050), "₱100.50");
        assert_eq!(Currency::PHP.format_amount(100), "₱1.00");
        assert_eq!(Currency::PHP.format_amount(0), "₱0.00");
        assert_eq!(Currency::USD.format_amount(12345), "$123.45");
    }

    #[test]
    fn test_format_amount_negative() {
        assert_eq!(Currency::PHP.format_amount(-10050), "₱-100.50");
    }

    #[test]
    fn test_currency_serialization() {
        let currency = Currency::PHP;
        let json = serde_json::to_string(&currency).unwrap();
        assert_eq!(json, "\"php\"");
    }

    #[test]
    fn test_currency_deserialization() {
        let json = "\"php\"";
        let currency: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(currency, Currency::PHP);
    }

    #[test]
    fn test_currency_default() {
        assert_eq!(Currency::default(), Currency::PHP);
    }
}
