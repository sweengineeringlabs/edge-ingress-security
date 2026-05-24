//! Hand-rolled protobuf codec for `grpc.reflection.v1alpha`.

use crate::api::error::ReflectionError;
use crate::api::types::{ReflectionRequest, ReflectionResponse};

/// Codec for gRPC reflection protobuf messages.
pub struct ReflectionCodec;

impl ReflectionCodec {
    /// Decode a `ServerReflectionRequest`.
    pub fn decode_request(body: &[u8]) -> Result<ReflectionRequest, ReflectionError> {
        let mut idx = 0usize;
        let mut found: Option<ReflectionRequest> = None;

        while idx < body.len() {
            let (tag, consumed) = Self::decode_varint(&body[idx..])
                .ok_or_else(|| ReflectionError::Malformed("varint tag".into()))?;
            idx += consumed;
            let field_number = (tag >> 3) as u32;
            let wire_type = (tag & 0x7) as u8;
            match (field_number, wire_type) {
                (1, 2) => {
                    let (len, c) = Self::decode_varint(&body[idx..])
                        .ok_or_else(|| ReflectionError::Malformed("host length".into()))?;
                    idx += c;
                    let end = idx
                        .checked_add(len as usize)
                        .ok_or_else(|| ReflectionError::Malformed("host overflow".into()))?;
                    if end > body.len() {
                        return Err(ReflectionError::Malformed("host truncated".into()));
                    }
                    idx = end;
                }
                (3, 2) => {
                    let (s, c) = Self::decode_string(&body[idx..])?;
                    idx += c;
                    found = Some(ReflectionRequest::FileByFilename(s));
                }
                (4, 2) => {
                    let (s, c) = Self::decode_string(&body[idx..])?;
                    idx += c;
                    found = Some(ReflectionRequest::FileContainingSymbol(s));
                }
                (5, 2) => {
                    let (len, c) = Self::decode_varint(&body[idx..])
                        .ok_or_else(|| ReflectionError::Malformed("ext-req length".into()))?;
                    idx += c;
                    let end = idx
                        .checked_add(len as usize)
                        .ok_or_else(|| ReflectionError::Malformed("ext-req overflow".into()))?;
                    if end > body.len() {
                        return Err(ReflectionError::Malformed("ext-req truncated".into()));
                    }
                    let (containing_type, extension_number) =
                        Self::decode_extension_request(&body[idx..end])?;
                    idx = end;
                    found = Some(ReflectionRequest::FileContainingExtension {
                        containing_type,
                        extension_number,
                    });
                }
                (6, 2) => {
                    let (s, c) = Self::decode_string(&body[idx..])?;
                    idx += c;
                    found = Some(ReflectionRequest::AllExtensionNumbersOfType(s));
                }
                (7, 2) => {
                    let (s, c) = Self::decode_string(&body[idx..])?;
                    idx += c;
                    found = Some(ReflectionRequest::ListServices(s));
                }
                (_, wt) => {
                    idx += Self::skip_field(&body[idx..], wt)?;
                }
            }
        }

        Ok(found.unwrap_or(ReflectionRequest::Unknown))
    }

    fn decode_extension_request(body: &[u8]) -> Result<(String, i32), ReflectionError> {
        let mut idx = 0usize;
        let mut containing_type = String::new();
        let mut extension_number = 0i32;
        while idx < body.len() {
            let (tag, c) = Self::decode_varint(&body[idx..])
                .ok_or_else(|| ReflectionError::Malformed("ext-req tag".into()))?;
            idx += c;
            let fnum = (tag >> 3) as u32;
            let wire = (tag & 0x7) as u8;
            match (fnum, wire) {
                (1, 2) => {
                    let (s, c) = Self::decode_string(&body[idx..])?;
                    idx += c;
                    containing_type = s;
                }
                (2, 0) => {
                    let (v, c) = Self::decode_varint(&body[idx..])
                        .ok_or_else(|| ReflectionError::Malformed("ext-req number".into()))?;
                    idx += c;
                    extension_number = v as i32;
                }
                (_, wt) => {
                    idx += Self::skip_field(&body[idx..], wt)?;
                }
            }
        }
        Ok((containing_type, extension_number))
    }

    /// Encode a `ServerReflectionResponse`.
    pub fn encode_response(response: &ReflectionResponse, original_request: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(64);
        if !original_request.is_empty() {
            out.push(0x12);
            Self::encode_varint(original_request.len() as u64, &mut out);
            out.extend_from_slice(original_request);
        }
        match response {
            ReflectionResponse::FileDescriptor(files) => {
                let mut sub = Vec::new();
                for f in files {
                    sub.push(0x0a);
                    Self::encode_varint(f.len() as u64, &mut sub);
                    sub.extend_from_slice(f);
                }
                out.push(0x22);
                Self::encode_varint(sub.len() as u64, &mut out);
                out.extend_from_slice(&sub);
            }
            ReflectionResponse::ListServices(names) => {
                let mut sub = Vec::new();
                for n in names {
                    let mut svc = Vec::new();
                    svc.push(0x0a);
                    Self::encode_varint(n.len() as u64, &mut svc);
                    svc.extend_from_slice(n.as_bytes());
                    sub.push(0x0a);
                    Self::encode_varint(svc.len() as u64, &mut sub);
                    sub.extend_from_slice(&svc);
                }
                out.push(0x32);
                Self::encode_varint(sub.len() as u64, &mut out);
                out.extend_from_slice(&sub);
            }
            ReflectionResponse::Error {
                error_code,
                error_message,
            } => {
                let mut sub = Vec::new();
                if *error_code != 0 {
                    sub.push(0x08);
                    Self::encode_varint(*error_code as i64 as u64, &mut sub);
                }
                if !error_message.is_empty() {
                    sub.push(0x12);
                    Self::encode_varint(error_message.len() as u64, &mut sub);
                    sub.extend_from_slice(error_message.as_bytes());
                }
                out.push(0x3a);
                Self::encode_varint(sub.len() as u64, &mut out);
                out.extend_from_slice(&sub);
            }
        }
        out
    }

    fn decode_string(body: &[u8]) -> Result<(String, usize), ReflectionError> {
        let (len, c) = Self::decode_varint(body)
            .ok_or_else(|| ReflectionError::Malformed("string length".into()))?;
        let total = c
            .checked_add(len as usize)
            .ok_or_else(|| ReflectionError::Malformed("string overflow".into()))?;
        if total > body.len() {
            return Err(ReflectionError::Malformed("string truncated".into()));
        }
        let s = std::str::from_utf8(&body[c..c + len as usize])
            .map_err(|_| ReflectionError::Malformed("string utf-8".into()))?
            .to_string();
        Ok((s, total))
    }

    fn skip_field(body: &[u8], wire_type: u8) -> Result<usize, ReflectionError> {
        match wire_type {
            0 => {
                let (_, c) = Self::decode_varint(body)
                    .ok_or_else(|| ReflectionError::Malformed("skip varint".into()))?;
                Ok(c)
            }
            1 => {
                if body.len() < 8 {
                    return Err(ReflectionError::Malformed("skip fixed64".into()));
                }
                Ok(8)
            }
            2 => {
                let (len, c) = Self::decode_varint(body)
                    .ok_or_else(|| ReflectionError::Malformed("skip ld length".into()))?;
                let total = c
                    .checked_add(len as usize)
                    .ok_or_else(|| ReflectionError::Malformed("skip ld overflow".into()))?;
                if total > body.len() {
                    return Err(ReflectionError::Malformed("skip ld truncated".into()));
                }
                Ok(total)
            }
            5 => {
                if body.len() < 4 {
                    return Err(ReflectionError::Malformed("skip fixed32".into()));
                }
                Ok(4)
            }
            wt => Err(ReflectionError::Malformed(format!(
                "unsupported wire type {wt}"
            ))),
        }
    }

    pub(crate) fn decode_varint(bytes: &[u8]) -> Option<(u64, usize)> {
        let mut result = 0u64;
        let mut shift = 0u32;
        for (i, byte) in bytes.iter().take(10).enumerate() {
            result |= ((byte & 0x7f) as u64) << shift;
            if byte & 0x80 == 0 {
                return Some((result, i + 1));
            }
            shift += 7;
        }
        None
    }

    pub(crate) fn encode_varint(mut value: u64, out: &mut Vec<u8>) {
        while value >= 0x80 {
            out.push((value as u8) | 0x80);
            value >>= 7;
        }
        out.push(value as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode_string_field(tag: u8, value: &str) -> Vec<u8> {
        let mut out = vec![tag];
        ReflectionCodec::encode_varint(value.len() as u64, &mut out);
        out.extend_from_slice(value.as_bytes());
        out
    }

    #[test]
    fn test_varint_round_trips_for_typical_values() {
        for v in [0u64, 1, 0x7f, 0x80, 0x3fff, 0xffff_ffff] {
            let mut buf = Vec::new();
            ReflectionCodec::encode_varint(v, &mut buf);
            let (out, c) = ReflectionCodec::decode_varint(&buf).unwrap();
            assert_eq!(out, v);
            assert_eq!(c, buf.len());
        }
    }

    #[test]
    fn test_decode_request_empty_body_returns_unknown() {
        assert_eq!(
            ReflectionCodec::decode_request(&[]).unwrap(),
            ReflectionRequest::Unknown
        );
    }

    #[test]
    fn test_decode_request_list_services_round_trips() {
        let body = encode_string_field(0x3a, "");
        assert_eq!(
            ReflectionCodec::decode_request(&body).unwrap(),
            ReflectionRequest::ListServices(String::new())
        );
    }

    #[test]
    fn test_decode_request_file_by_filename_round_trips() {
        let body = encode_string_field(0x1a, "pkg/foo.proto");
        assert_eq!(
            ReflectionCodec::decode_request(&body).unwrap(),
            ReflectionRequest::FileByFilename("pkg/foo.proto".into())
        );
    }
}
