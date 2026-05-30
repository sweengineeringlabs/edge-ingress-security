//! Contract tests for ApplicationConfigBuilder type alias.

use swe_edge_configbuilder::ConfigBuilderImpl;
use swe_edge_ingress_verifier::{ApplicationConfigBuilder, VerifierSvc};

/// @covers: create_config_builder
#[test]
fn test_application_config_builder_is_config_builder_impl() {
    let builder: ApplicationConfigBuilder = VerifierSvc::create_config_builder();
    assert_eq!(builder.name(), env!("CARGO_PKG_NAME"));
}

/// @covers: ApplicationConfigBuilder
#[test]
fn test_application_config_builder_alias_matches_configbuilder_impl() {
    let _b: ConfigBuilderImpl = VerifierSvc::create_config_builder();
}
