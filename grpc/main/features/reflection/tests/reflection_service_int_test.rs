//! Integration tests for `ReflectionService` builder methods and helpers.
//!
//! Rules 77 + 78: covers `with_descriptors`, `ReflectionService::service_name_from_method_path`, and
//! `add_descriptor` with `/// @covers:` annotations.

use std::sync::Arc;

use edge_domain::HandlerRegistry;
use swe_edge_ingress_grpc_reflection::{
    handle_reflection, Descriptor, ReflectionRequest, ReflectionResponse, ReflectionService,
    REFLECTION_SERVICE_NAME,
};

fn make_registry() -> Arc<HandlerRegistry<Vec<u8>, Vec<u8>>> {
    Arc::new(HandlerRegistry::new())
}

// ── with_descriptors ──────────────────────────────────────────────────────────

/// @covers: ReflectionService::with_descriptors — single descriptor is findable by filename.
#[test]
fn test_with_descriptors_single_descriptor_findable_by_filename() {
    let svc = ReflectionService::new(make_registry()).with_descriptors(vec![Descriptor {
        filename: "pkg/a.proto".into(),
        symbols: vec!["pkg.A".into()],
        bytes: vec![0xaa, 0xbb],
    }]);
    let resp = handle_reflection(
        &svc,
        ReflectionRequest::FileByFilename("pkg/a.proto".into()),
    );
    assert!(
        matches!(resp, ReflectionResponse::FileDescriptor(_)),
        "expected FileDescriptor, got {resp:?}"
    );
}

/// @covers: ReflectionService::with_descriptors — empty iterator leaves descriptor list empty.
#[test]
fn test_with_descriptors_empty_iterator_yields_not_found() {
    let svc = ReflectionService::new(make_registry()).with_descriptors(vec![]);
    let resp = handle_reflection(
        &svc,
        ReflectionRequest::FileByFilename("anything.proto".into()),
    );
    match resp {
        ReflectionResponse::Error { error_code, .. } => {
            assert_eq!(
                error_code,
                swe_edge_ingress_grpc_reflection::ERROR_CODE_NOT_FOUND,
                "expected NOT_FOUND for missing descriptor"
            );
        }
        other => panic!("expected Error(NOT_FOUND), got {other:?}"),
    }
}

/// @covers: ReflectionService::with_descriptors — multiple descriptors all findable by symbol.
#[test]
fn test_with_descriptors_multiple_descriptors_findable_by_symbol() {
    let svc = ReflectionService::new(make_registry()).with_descriptors(vec![
        Descriptor {
            filename: "a.proto".into(),
            symbols: vec!["pkg.A".into()],
            bytes: vec![0x0a],
        },
        Descriptor {
            filename: "b.proto".into(),
            symbols: vec!["pkg.B".into()],
            bytes: vec![0x0b],
        },
    ]);

    let resp_a = handle_reflection(
        &svc,
        ReflectionRequest::FileContainingSymbol("pkg.A".into()),
    );
    assert!(
        matches!(resp_a, ReflectionResponse::FileDescriptor(_)),
        "expected FileDescriptor for pkg.A, got {resp_a:?}"
    );

    let resp_b = handle_reflection(
        &svc,
        ReflectionRequest::FileContainingSymbol("pkg.B".into()),
    );
    assert!(
        matches!(resp_b, ReflectionResponse::FileDescriptor(_)),
        "expected FileDescriptor for pkg.B, got {resp_b:?}"
    );
}

// ── ReflectionService::service_name_from_method_path ─────────────────────────────────────────────

/// @covers: ReflectionService::service_name_from_method_path — well-formed path returns service segment.
#[test]
fn test_service_name_from_method_path_well_formed_returns_service_segment() {
    assert_eq!(
        ReflectionService::service_name_from_method_path("/grpc.health.v1.Health/Check"),
        Some("grpc.health.v1.Health")
    );
}

/// @covers: ReflectionService::service_name_from_method_path — path without leading slash returns None.
#[test]
fn test_service_name_from_method_path_no_leading_slash_returns_none() {
    assert!(ReflectionService::service_name_from_method_path("no.slash/Method").is_none());
}

/// @covers: ReflectionService::service_name_from_method_path — empty string returns None.
#[test]
fn test_service_name_from_method_path_empty_string_returns_none() {
    assert!(ReflectionService::service_name_from_method_path("").is_none());
}

/// @covers: ReflectionService::service_name_from_method_path — slash-only string returns None (empty service name).
#[test]
fn test_service_name_from_method_path_slash_only_returns_none() {
    assert!(ReflectionService::service_name_from_method_path("/").is_none());
}

/// @covers: ReflectionService::service_name_from_method_path — reflection own method path returns REFLECTION_SERVICE_NAME.
#[test]
fn test_service_name_from_method_path_reflection_path_returns_reflection_service_name() {
    let result = ReflectionService::service_name_from_method_path(
        "/grpc.reflection.v1alpha.ServerReflection/ServerReflectionInfo",
    );
    assert_eq!(result, Some(REFLECTION_SERVICE_NAME));
}

// ── add_descriptor ────────────────────────────────────────────────────────────

/// @covers: ReflectionService::add_descriptor — chained calls accumulate descriptors.
#[test]
fn test_add_descriptor_chained_calls_accumulate_descriptors() {
    let svc = ReflectionService::new(make_registry())
        .add_descriptor(Descriptor {
            filename: "first.proto".into(),
            symbols: vec!["pkg.First".into()],
            bytes: vec![0x01],
        })
        .add_descriptor(Descriptor {
            filename: "second.proto".into(),
            symbols: vec!["pkg.Second".into()],
            bytes: vec![0x02],
        });

    let r1 = handle_reflection(
        &svc,
        ReflectionRequest::FileByFilename("first.proto".into()),
    );
    let r2 = handle_reflection(
        &svc,
        ReflectionRequest::FileByFilename("second.proto".into()),
    );

    assert!(
        matches!(r1, ReflectionResponse::FileDescriptor(_)),
        "first.proto must be found"
    );
    assert!(
        matches!(r2, ReflectionResponse::FileDescriptor(_)),
        "second.proto must be found"
    );
}

/// @covers: ReflectionService::add_descriptor — descriptor bytes are returned verbatim.
#[test]
fn test_add_descriptor_bytes_returned_verbatim() {
    let magic: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef];
    let svc = ReflectionService::new(make_registry()).add_descriptor(Descriptor {
        filename: "magic.proto".into(),
        symbols: vec![],
        bytes: magic.clone(),
    });
    let resp = handle_reflection(
        &svc,
        ReflectionRequest::FileByFilename("magic.proto".into()),
    );
    match resp {
        ReflectionResponse::FileDescriptor(files) => {
            assert_eq!(files.len(), 1);
            assert_eq!(
                files[0], magic,
                "descriptor bytes must be returned verbatim"
            );
        }
        other => panic!("expected FileDescriptor, got {other:?}"),
    }
}
