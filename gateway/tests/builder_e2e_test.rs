//! End-to-end tests for the swe_edge_ingress SAF builder surface.

use swe_edge_ingress::{file_input, InboundSource};

/// @covers: file_input
#[test]
fn test_file_input_builds_and_checks_existing_path() {
    let src = file_input();
    let exists = src.file_exists(std::path::Path::new(".")).unwrap();
    assert!(exists);
}
