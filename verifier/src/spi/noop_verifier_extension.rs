//! [`NoopVerifierExtension`] default extension — marker impl for spi layer.

/// Primary type for this module — satisfies Rule 89 filename match.
#[expect(
    dead_code,
    reason = "SEA spi/ extension anchor — never directly constructed"
)]
pub(crate) struct DefaultNoopVerifierExtension;
