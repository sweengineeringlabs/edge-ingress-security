//! Dep coverage for tower.
use tower::Layer;
/// @covers: tower
#[test]
fn verifier_trait_tower_dep_layer_is_accessible_int_test() {
    fn _assert<T: Layer<()>>() {}
}
