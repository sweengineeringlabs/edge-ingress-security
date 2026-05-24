//! Fluent builder for [`HttpRequest`].

use std::collections::HashMap;
use std::time::Duration;

use super::http_request::HttpRequest;
use crate::api::value::http::http_body::HttpBody;
use crate::api::value::http::http_method::HttpMethod;

/// Fluent builder that constructs an [`HttpRequest`].
pub struct HttpRequestBuilder {
    method: HttpMethod,
    url: String,
    headers: HashMap<String, String>,
    query: HashMap<String, String>,
    body: Option<HttpBody>,
    timeout: Option<Duration>,
}

impl HttpRequestBuilder {
    /// Start a builder for a GET request to `url`.
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Get,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Start a builder for a POST request to `url`.
    pub fn post(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Post,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Add a request header.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Add a query parameter.
    pub fn with_query(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(name.into(), value.into());
        self
    }

    /// Set the request body.
    pub fn with_body(mut self, body: HttpBody) -> Self {
        self.body = Some(body);
        self
    }

    /// Set the per-request timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Consume the builder and return an [`HttpRequest`].
    pub fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url: self.url,
            headers: self.headers,
            query: self.query,
            body: self.body,
            timeout: self.timeout,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: get
    #[test]
    fn test_get_creates_builder_for_get_request() {
        let req = HttpRequestBuilder::get("/ping").build();
        assert_eq!(req.method, HttpMethod::Get);
        assert_eq!(req.url, "/ping");
    }

    /// @covers: post
    #[test]
    fn test_post_creates_builder_for_post_request() {
        let req = HttpRequestBuilder::post("/submit").build();
        assert_eq!(req.method, HttpMethod::Post);
    }

    /// @covers: with_header
    #[test]
    fn test_with_header_adds_header_to_request() {
        let req = HttpRequestBuilder::get("/")
            .with_header("Authorization", "Bearer tok")
            .build();
        assert_eq!(
            req.headers.get("Authorization").map(|s| s.as_str()),
            Some("Bearer tok")
        );
    }

    /// @covers: with_query
    #[test]
    fn test_with_query_adds_query_param_to_request() {
        let req = HttpRequestBuilder::get("/").with_query("page", "2").build();
        assert_eq!(req.query.get("page").map(|s| s.as_str()), Some("2"));
    }

    /// @covers: with_body
    #[test]
    fn test_with_body_sets_request_body() {
        let req = HttpRequestBuilder::post("/")
            .with_body(HttpBody::Raw(b"hello".to_vec()))
            .build();
        assert!(matches!(req.body, Some(HttpBody::Raw(_))));
    }

    /// @covers: with_timeout
    #[test]
    fn test_with_timeout_sets_request_timeout() {
        let req = HttpRequestBuilder::get("/")
            .with_timeout(Duration::from_secs(10))
            .build();
        assert_eq!(req.timeout, Some(Duration::from_secs(10)));
    }

    /// @covers: build
    #[test]
    fn test_build_returns_http_request_with_set_values() {
        let req = HttpRequestBuilder::get("/api")
            .with_header("X-Test", "1")
            .build();
        assert_eq!(req.url, "/api");
        assert!(req.headers.contains_key("X-Test"));
    }
}
