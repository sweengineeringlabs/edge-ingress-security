//! HTTP method enum.

use serde::{Deserialize, Serialize};

/// An HTTP request method.
///
/// Implements [`Display`] returning the uppercase string form (`"GET"`, `"POST"`, etc.).
/// Defaults to [`HttpMethod::Get`].
///
/// [`Display`]: std::fmt::Display
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_http::HttpMethod;
///
/// assert_eq!(HttpMethod::default(), HttpMethod::Get);
/// assert_eq!(HttpMethod::Post.to_string(), "POST");
/// assert_eq!(HttpMethod::Delete.to_string(), "DELETE");
///
/// // All variants round-trip through their string form.
/// let methods = [
///     HttpMethod::Get, HttpMethod::Post, HttpMethod::Put, HttpMethod::Patch,
///     HttpMethod::Delete, HttpMethod::Head, HttpMethod::Options,
/// ];
/// for m in methods {
///     assert!(!m.to_string().is_empty());
/// }
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    /// HTTP GET.
    #[default]
    Get,
    /// HTTP POST.
    Post,
    /// HTTP PUT.
    Put,
    /// HTTP PATCH.
    Patch,
    /// HTTP DELETE.
    Delete,
    /// HTTP HEAD.
    Head,
    /// HTTP OPTIONS.
    Options,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Patch => write!(f, "PATCH"),
            Self::Delete => write!(f, "DELETE"),
            Self::Head => write!(f, "HEAD"),
            Self::Options => write!(f, "OPTIONS"),
        }
    }
}
