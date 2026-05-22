//! Dependency-coverage test for `futures`.
//!
//! `futures` is listed in `[dependencies]` to support async stream
//! composition in higher-level consumers.  This test exercises the
//! crate's `futures::executor::block_on` helper to confirm the dep
//! resolves and compiles correctly within this workspace.

use futures::executor::block_on;

/// @covers: futures dependency — async block can be executed synchronously
#[test]
fn test_futures_block_on_resolves_async_block() {
    let result = block_on(async { 42u32 });
    assert_eq!(
        result, 42,
        "block_on must return the value produced by the async block"
    );
}

/// @covers: futures dependency — ready future resolves immediately
#[test]
fn test_futures_future_ready_resolves_value() {
    use futures::future::ready;
    let value = block_on(ready("bearer"));
    assert_eq!(value, "bearer");
}
