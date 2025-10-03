//! HTTP client implementation with retry logic and error handling.
//!
//! This module provides a wrapper around `reqwest` with automatic retries,
//! rate limiting, and proper error handling for the PayRex API.

use crate::{Config, Error, ErrorKind, Result};
use base64::{Engine as _, engine::general_purpose};
use reqwest::{Client as ReqwestClient, RequestBuilder, Response, StatusCode, header};
use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;

/// HTTP client for making requests to the PayRex API.
pub(crate) struct HttpClient {
    client: ReqwestClient,
    config: Config,
}

impl HttpClient {
    pub fn new(config: Config) -> Result<Self> {
        let mut headers = header::HeaderMap::new();

        // Use HTTP Basic Authentication (API key as username, empty password)
        let credentials = format!("{}:", config.api_key());
        let encoded = general_purpose::STANDARD.encode(credentials.as_bytes());
        let auth_value = format!("Basic {}", encoded);
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth_value)
                .map_err(|e| Error::Config(format!("Invalid API key format: {e}")))?,
        );

        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(config.user_agent())
                .map_err(|e| Error::Config(format!("Invalid user agent: {e}")))?,
        );

        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let client = ReqwestClient::builder()
            .default_headers(headers)
            .timeout(config.timeout())
            .build()
            .map_err(|e| Error::Config(format!("Failed to build HTTP client: {e}")))?;

        Ok(Self { client, config })
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.build_url(path)?;
        self.execute_with_retry_get(&url).await
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let url = self.build_url(path)?;
        let serialized = serde_json::to_value(body)
            .map_err(|e| Error::Config(format!("Failed to serialize request body: {e}")))?;
        self.execute_with_retry_form(&url, "POST", serialized).await
    }

    #[allow(dead_code)]
    pub async fn put<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let url = self.build_url(path)?;
        let serialized = serde_json::to_value(body)
            .map_err(|e| Error::Config(format!("Failed to serialize request body: {e}")))?;
        self.execute_with_retry_form(&url, "PUT", serialized).await
    }

    pub async fn patch<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let serialized = serde_json::to_value(body)
            .map_err(|e| Error::Config(format!("Failed to serialize request body: {e}")))?;
        self.execute_with_retry_form(&url, "PATCH", serialized)
            .await
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.build_url(path)?;
        self.execute_with_retry_get(&url).await
    }

    fn build_url(&self, path: &str) -> Result<String> {
        let base = self.config.api_base_url().trim_end_matches('/');
        let path = path.trim_start_matches('/');
        Ok(format!("{base}/{path}"))
    }

    async fn execute_with_retry_get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let mut attempts = 0;
        let max_retries = self.config.max_retries();

        loop {
            let request = self.client.get(url);

            match self.execute_request(request).await {
                Ok(response) => return self.handle_response(response).await,
                Err(e) if e.is_retryable() && attempts < max_retries => {
                    attempts += 1;
                    let delay = self.calculate_retry_delay(attempts);
                    tokio::time::sleep(delay).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    async fn execute_with_retry_form<T: DeserializeOwned>(
        &self,
        url: &str,
        method: &str,
        body: serde_json::Value,
    ) -> Result<T> {
        let mut attempts = 0;
        let max_retries = self.config.max_retries();

        loop {
            let request = match method {
                "POST" => self.client.post(url),
                "PUT" => self.client.put(url),
                "PATCH" => self.client.patch(url),
                _ => return Err(Error::Internal(format!("Unsupported method: {method}"))),
            };

            // Convert JSON value to form data
            let request = if let Some(obj) = body.as_object() {
                let mut form = Vec::new();
                self.flatten_json_to_form("", obj, &mut form);
                request.form(&form)
            } else {
                request.form(&body)
            };

            match self.execute_request(request).await {
                Ok(response) => return self.handle_response(response).await,
                Err(e) if e.is_retryable() && attempts < max_retries => {
                    attempts += 1;
                    let delay = self.calculate_retry_delay(attempts);
                    tokio::time::sleep(delay).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn flatten_json_to_form(
        &self,
        prefix: &str,
        obj: &serde_json::Map<String, serde_json::Value>,
        form: &mut Vec<(String, String)>,
    ) {
        for (key, value) in obj {
            let field_name = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}[{}]", prefix, key)
            };

            match value {
                serde_json::Value::Object(nested) => {
                    self.flatten_json_to_form(&field_name, nested, form);
                }
                serde_json::Value::Array(arr) => {
                    for (i, item) in arr.iter().enumerate() {
                        if let serde_json::Value::Object(nested) = item {
                            self.flatten_json_to_form(
                                &format!("{}[{}]", field_name, i),
                                nested,
                                form,
                            );
                        } else {
                            form.push((
                                format!("{}[{}]", field_name, i),
                                item.to_string().trim_matches('"').to_string(),
                            ));
                        }
                    }
                }
                serde_json::Value::String(s) => {
                    form.push((field_name, s.clone()));
                }
                serde_json::Value::Number(n) => {
                    form.push((field_name, n.to_string()));
                }
                serde_json::Value::Bool(b) => {
                    form.push((field_name, b.to_string()));
                }
                serde_json::Value::Null => {
                    // Skip null values
                }
            }
        }
    }

    async fn execute_request(&self, request: RequestBuilder) -> Result<Response> {
        request.send().await.map_err(|e| {
            if e.is_timeout() {
                Error::Timeout(self.config.timeout())
            } else {
                Error::Http(e)
            }
        })
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();
        let request_id = response
            .headers()
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        if status == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .map(Duration::from_secs);

            return Err(Error::RateLimit { retry_after });
        }

        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_default();
            let kind = Self::status_to_error_kind(status);

            return Err(Error::Api {
                kind,
                message: error_body,
                status_code: Some(status.as_u16()),
                request_id,
            });
        }

        response.json().await.map_err(Error::Http)
    }

    fn status_to_error_kind(status: StatusCode) -> ErrorKind {
        match status {
            StatusCode::BAD_REQUEST => ErrorKind::InvalidRequest,
            StatusCode::UNAUTHORIZED => ErrorKind::Authentication,
            StatusCode::FORBIDDEN => ErrorKind::PermissionDenied,
            StatusCode::NOT_FOUND => ErrorKind::NotFound,
            StatusCode::TOO_MANY_REQUESTS => ErrorKind::RateLimit,
            s if s.is_server_error() => ErrorKind::ServerError,
            _ => ErrorKind::Unknown,
        }
    }

    fn calculate_retry_delay(&self, attempt: u32) -> Duration {
        let base_delay = self.config.retry_delay();
        let multiplier = 2_u32.pow(attempt.saturating_sub(1));
        base_delay * multiplier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url() {
        let config = Config::new("test_key").unwrap();
        let client = HttpClient::new(config).unwrap();

        let url = client.build_url("/payment_intents").unwrap();
        assert!(url.contains("payment_intents"));
        assert!(url.starts_with("https://"));
    }

    #[test]
    fn test_calculate_retry_delay() {
        let config = Config::builder()
            .api_key("test_key")
            .retry_delay(Duration::from_millis(100))
            .build()
            .unwrap();
        let client = HttpClient::new(config).unwrap();

        assert_eq!(client.calculate_retry_delay(1), Duration::from_millis(100));
        assert_eq!(client.calculate_retry_delay(2), Duration::from_millis(200));
        assert_eq!(client.calculate_retry_delay(3), Duration::from_millis(400));
    }

    #[test]
    fn test_status_to_error_kind() {
        assert_eq!(
            HttpClient::status_to_error_kind(StatusCode::BAD_REQUEST),
            ErrorKind::InvalidRequest
        );
        assert_eq!(
            HttpClient::status_to_error_kind(StatusCode::UNAUTHORIZED),
            ErrorKind::Authentication
        );
        assert_eq!(
            HttpClient::status_to_error_kind(StatusCode::NOT_FOUND),
            ErrorKind::NotFound
        );
    }
}
