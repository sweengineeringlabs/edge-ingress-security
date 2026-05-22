//! `Descriptor` — snapshot of a registered `FileDescriptorProto`.

use serde::{Deserialize, Serialize};

/// Snapshot of a registered descriptor — paired with the source bytes
/// of a `FileDescriptorProto`.
///
/// The reflection service stores one `Descriptor` per registered file;
/// `filename` and `symbols` are pre-extracted at registration time so
/// per-request lookup is O(1) under a read lock.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    /// `FileDescriptorProto.name` (e.g. `"pkg/foo.proto"`).
    pub filename: String,
    /// Fully-qualified symbols defined in this file —
    /// `pkg.Service`, `pkg.Service.Method`, `pkg.Message`, etc.
    pub symbols: Vec<String>,
    /// Raw `FileDescriptorProto` bytes — what reflection clients receive verbatim.
    pub bytes: Vec<u8>,
}
