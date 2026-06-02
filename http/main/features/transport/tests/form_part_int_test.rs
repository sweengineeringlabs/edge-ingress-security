//! Tests for FormPart.

use swe_edge_ingress_http::FormPart;

#[test]
fn test_form_part_construct() {
    let part = FormPart {
        name: "field".to_string(),
        filename: None,
        content_type: None,
        data: vec![],
    };
    assert_eq!(part.name, "field");
}
