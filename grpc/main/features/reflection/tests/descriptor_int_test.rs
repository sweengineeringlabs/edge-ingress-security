//! Integration tests for `Descriptor` fields and serialization round-trip.

use swe_edge_ingress_grpc_reflection::Descriptor;

/// @covers: Descriptor — all fields are stored and retrieved correctly.
#[test]
fn test_descriptor_fields_stored_and_retrieved_correctly() {
    let d = Descriptor {
        filename: "pkg/svc.proto".into(),
        symbols: vec!["pkg.Svc".into(), "pkg.Svc.Call".into()],
        bytes: vec![0x01, 0x02, 0x03],
    };
    assert_eq!(d.filename, "pkg/svc.proto");
    assert_eq!(d.symbols, vec!["pkg.Svc", "pkg.Svc.Call"]);
    assert_eq!(d.bytes, vec![0x01, 0x02, 0x03]);
}

/// @covers: Descriptor — Clone produces structurally equal value.
#[test]
fn test_descriptor_clone_produces_equal_value() {
    let original = Descriptor {
        filename: "original.proto".into(),
        symbols: vec!["pkg.Orig".into()],
        bytes: vec![0xff],
    };
    let cloned = original.clone();
    assert_eq!(original.filename, cloned.filename);
    assert_eq!(original.symbols, cloned.symbols);
    assert_eq!(original.bytes, cloned.bytes);
}

/// @covers: Descriptor — empty symbols list is valid.
#[test]
fn test_descriptor_empty_symbols_list_is_valid() {
    let d = Descriptor {
        filename: "no_symbols.proto".into(),
        symbols: vec![],
        bytes: vec![0xab],
    };
    assert!(d.symbols.is_empty());
    assert_eq!(d.filename, "no_symbols.proto");
}

/// @covers: Descriptor — empty bytes field is valid.
#[test]
fn test_descriptor_empty_bytes_field_is_valid() {
    let d = Descriptor {
        filename: "empty_bytes.proto".into(),
        symbols: vec!["pkg.Empty".into()],
        bytes: vec![],
    };
    assert!(d.bytes.is_empty());
    assert_eq!(d.symbols[0], "pkg.Empty");
}

/// @covers: Descriptor — serde round-trip preserves all fields.
#[test]
fn test_descriptor_serde_round_trip_preserves_all_fields() {
    let original = Descriptor {
        filename: "serde_test.proto".into(),
        symbols: vec!["pkg.SerdeTest".into()],
        bytes: vec![0x01, 0x02],
    };
    let json = serde_json::to_string(&original).expect("serialize must succeed");
    let restored: Descriptor = serde_json::from_str(&json).expect("deserialize must succeed");
    assert_eq!(original.filename, restored.filename);
    assert_eq!(original.symbols, restored.symbols);
    assert_eq!(original.bytes, restored.bytes);
}
