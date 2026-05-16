//! Integration tests for `JwtVerifier`.

use swe_edge_ingress_verifier::{JwtConfig, JwtKey, JwtVerifier, TokenVerifier, VerifierError};

fn make_token(secret: &[u8], sub: &str, exp_offset_secs: i64) -> String {
    use jsonwebtoken::{encode, EncodingKey, Header};
    #[derive(serde::Serialize)]
    struct Raw {
        sub: String,
        exp: u64,
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let raw = Raw {
        sub: sub.into(),
        exp: (now as i64 + exp_offset_secs) as u64,
    };
    encode(&Header::default(), &raw, &EncodingKey::from_secret(secret)).unwrap()
}

fn hs256_config(secret: &[u8]) -> JwtConfig {
    JwtConfig {
        key: JwtKey::Hs256 {
            secret: secret.to_vec(),
        },
        required_issuer: None,
        required_audience: None,
        leeway_seconds: 0,
    }
}

/// @covers: JwtVerifier — valid HS256 token produces correct sub claim.
#[test]
fn test_jwt_verifier_valid_token_returns_sub() {
    let secret = b"integration-secret";
    let v = JwtVerifier::from_config(&hs256_config(secret)).unwrap();
    let token = make_token(secret, "carol", 3600);
    let claims = v.verify(&token).expect("valid token");
    assert_eq!(claims.sub.as_deref(), Some("carol"));
}

/// @covers: JwtVerifier — expired token returns VerifierError::Expired.
#[test]
fn test_jwt_verifier_expired_token_returns_expired() {
    let secret = b"exp-secret";
    let v = JwtVerifier::from_config(&hs256_config(secret)).unwrap();
    let token = make_token(secret, "x", -5);
    assert!(matches!(v.verify(&token), Err(VerifierError::Expired)));
}

/// @covers: JwtVerifier — wrong secret returns VerifierError::Invalid.
#[test]
fn test_jwt_verifier_wrong_secret_returns_invalid() {
    let v = JwtVerifier::from_config(&hs256_config(b"correct-secret")).unwrap();
    let token = make_token(b"wrong-secret", "x", 3600);
    assert!(matches!(v.verify(&token), Err(VerifierError::Invalid(_))));
}

/// @covers: JwtVerifier — issuer validation rejects mismatched iss.
#[test]
fn test_jwt_verifier_iss_mismatch_returns_claim_mismatch() {
    use jsonwebtoken::{encode, EncodingKey, Header};
    #[derive(serde::Serialize)]
    struct Raw {
        sub: String,
        iss: String,
        exp: u64,
    }
    let secret = b"iss-secret";
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let token = encode(
        &Header::default(),
        &Raw {
            sub: "u".into(),
            iss: "wrong.issuer".into(),
            exp: now + 3600,
        },
        &EncodingKey::from_secret(secret),
    )
    .unwrap();
    let cfg = JwtConfig {
        key: JwtKey::Hs256 {
            secret: secret.to_vec(),
        },
        required_issuer: Some("correct.issuer".into()),
        required_audience: None,
        leeway_seconds: 0,
    };
    let v = JwtVerifier::from_config(&cfg).unwrap();
    assert!(matches!(v.verify(&token), Err(VerifierError::ClaimMismatch(ref s)) if s == "iss"));
}
