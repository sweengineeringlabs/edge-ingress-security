//! Multipart form part type.

use serde::{Deserialize, Serialize};

/// A part of a multipart form.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormPart {
    /// Form field name.
    pub name: String,
    /// File name when the part is a file upload.
    pub filename: Option<String>,
    /// MIME type of this part.
    pub content_type: Option<String>,
    /// Raw bytes of this part.
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_part_constructs_with_name_and_data() {
        let part = FormPart {
            name: "file".to_string(),
            filename: Some("upload.txt".to_string()),
            content_type: Some("text/plain".to_string()),
            data: b"hello".to_vec(),
        };
        assert_eq!(part.name, "file");
        assert_eq!(part.filename.as_deref(), Some("upload.txt"));
    }
}
