//! Tests for http_config_validator_contract.
//!
//! `HttpConfigValidatorContract` is an internal SEA Rule 121 interface anchor
//! (not part of the public facade), so this compile contract guards the public
//! validator type the contract describes — `HttpConfigValidator`.
/// @covers: HttpConfigValidatorContract
#[test]
fn transport_trait_http_config_validator_contract_type_exists_int_test() {
    let _ = std::any::type_name::<swe_edge_ingress_http::HttpConfigValidator>();
}
