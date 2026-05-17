//! Integration tests for the local-filesystem InboundSource via the SAF.

use std::path::Path;

use swe_edge_ingress::{file_input, InboundSource};

/// @covers: file_input — returns InboundSource that reports an existing path as existing.
#[test]
fn test_file_input_detects_current_directory_as_existing() {
    let src = file_input();
    let exists = src
        .file_exists(Path::new("."))
        .expect("file_exists must not return an error for a reachable path");
    assert!(exists, "current directory must exist");
}

/// @covers: file_input — returns InboundSource that reports a missing path as absent.
#[test]
fn test_file_input_returns_false_for_nonexistent_path() {
    let src = file_input();
    let exists = src
        .file_exists(Path::new("/tmp/__swe_edge_ingress_nonexistent_xqz_1749__"))
        .expect("file_exists must not error for a readable path");
    assert!(!exists, "nonexistent path must return false");
}
