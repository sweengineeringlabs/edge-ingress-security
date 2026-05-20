//! Error type for HTTP handler dispatch operations.

/// Error returned when a handler registration fails.
#[derive(Debug, thiserror::Error)]
pub enum HttpDispatcherError {
    /// Route pattern could not be registered.
    #[error("failed to register pattern `{pattern}`: {reason}")]
    RegistrationFailed {
        /// The route pattern that failed to register.
        pattern: String,
        /// The reason the registration was rejected.
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_dispatcher_error_formats_with_pattern_and_reason() {
        let e = HttpDispatcherError::RegistrationFailed {
            pattern: "/api/v1".into(),
            reason: "conflict".into(),
        };
        let msg = e.to_string();
        assert!(msg.contains("/api/v1"), "{msg}");
        assert!(msg.contains("conflict"), "{msg}");
    }
}
