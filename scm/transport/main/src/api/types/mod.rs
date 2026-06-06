//! Cross-theme HTTP transport types.

pub mod transport_svc;
pub use transport_svc::TransportSvc;

pub mod http_health_check;
pub mod http_ingress_result;
pub use http_health_check::HttpHealthCheck;
pub use http_ingress_result::HttpIngressResult;
