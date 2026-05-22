//! Minimal example: construct a `ReflectionService` and print the well-known
//! method path constant.
//!
//! Run with:
//!   cargo run --example basic_reflection

fn main() {
    println!(
        "Reflection method path: {}",
        swe_edge_ingress_grpc_reflection::REFLECTION_INFO_METHOD
    );
}
