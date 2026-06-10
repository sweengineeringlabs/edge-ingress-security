//! [`TenantId`] — opaque tenant identity token.

/// Opaque tenant identifier extracted from an inbound request.
///
/// Wraps a UTF-8 string and participates in routing decisions (e.g. feeding
/// `IngressLoadBalancer::on_accept`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct TenantId(String);

impl TenantId {
    /// Construct a [`TenantId`] from any string-like value.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the inner tenant identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TenantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for TenantId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for TenantId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}
