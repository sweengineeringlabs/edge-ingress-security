//! API layer — verifier traits, config, and value objects.

pub mod api_key_verifier;
pub mod application_config_builder;
pub mod architecture_config_builder;
pub mod claims;
pub mod jwt_config;
pub mod jwt_verifier;
pub mod token_verifier;
pub mod verifier_error;

pub use application_config_builder::ApplicationConfigBuilder;
pub use architecture_config_builder::ArchitectureConfigBuilder;
