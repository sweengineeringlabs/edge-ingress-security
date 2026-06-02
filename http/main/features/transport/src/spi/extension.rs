//! Extension point marker for downstream HTTP transport consumers.

/// Extension point marker — downstream consumers implement [`HttpTransportConfigSection`](crate::api::traits::HttpTransportConfigSection).
#[expect(
    dead_code,
    reason = "SEA spi/ anchor — intentionally unused in this crate"
)]
pub(crate) struct Extension;
