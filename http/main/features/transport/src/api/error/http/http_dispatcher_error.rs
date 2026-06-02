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
