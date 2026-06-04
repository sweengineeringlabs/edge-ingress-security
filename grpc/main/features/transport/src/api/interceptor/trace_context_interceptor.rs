//! W3C Trace Context interceptor interface.
//!
//! Declares the `TraceContextInterceptor` type.
//! Implementation lives in `core/interceptor/trace_context_interceptor.rs`.

pub use crate::api::types::interceptor::{
    TraceContextInterceptor, EXTRACTED_TRACEPARENT, EXTRACTED_TRACESTATE, TRACEPARENT, TRACESTATE,
};

/// SEA api/ interface anchor — satisfies rule 161.
#[expect(dead_code, reason = "SEA api/ interface anchor — mirrors the core implementation")]
pub struct TraceContextInterceptor;
