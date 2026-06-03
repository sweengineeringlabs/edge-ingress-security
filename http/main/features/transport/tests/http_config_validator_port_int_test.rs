//! Tests for http_config_validator_port.
//!
//! `HttpConfigValidatorPort` is an internal SEA Rule 121 interface anchor
//! (not part of the public facade), so this compile contract guards the public
//! validator type the port describes — `HttpConfigValidator`.
/// @covers: HttpConfigValidatorPort
#[test]
fn transport_trait_http_config_validator_port_type_exists_int_test() {
    let _ = std::any::type_name::<swe_edge_ingress_http::HttpConfigValidator>();
}
