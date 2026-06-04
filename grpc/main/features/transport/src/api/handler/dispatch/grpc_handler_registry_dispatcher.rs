//! Handler dispatch interface — declares the types that core/handler_dispatch implements.


/// SEA api/ interface anchor — satisfies rule 161 (one pub type per file).
///
/// The actual implementation lives in the corresponding  module.
#[expect(dead_code, reason = "SEA api/ interface anchor — mirrors the core implementation")]
pub struct GrpcHandlerRegistryDispatcher;
