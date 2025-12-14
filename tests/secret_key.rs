use greentic_types::secrets::{SecretKey, SecretKeyError};

#[test]
fn parses_valid_keys() {
    assert!(SecretKey::parse("PRIMARY_TOKEN").is_ok());
    assert!(SecretKey::parse("nested/path-1._").is_ok());
    assert!(SecretKey::parse("a.b_c-d/e").is_ok());
}

#[test]
fn rejects_empty_keys() {
    assert!(matches!(SecretKey::parse(""), Err(SecretKeyError::Empty)));
}

#[test]
fn rejects_leading_slash() {
    assert!(matches!(
        SecretKey::parse("/ROOTED"),
        Err(SecretKeyError::LeadingSlash)
    ));
}

#[test]
fn rejects_dotdot_segment() {
    assert!(matches!(
        SecretKey::parse("valid/../bad"),
        Err(SecretKeyError::DotDotSegment)
    ));
}

#[test]
fn rejects_invalid_characters() {
    assert!(matches!(
        SecretKey::parse("bad space"),
        Err(SecretKeyError::InvalidChar { .. })
    ));
    assert!(matches!(
        SecretKey::parse("bad:colon"),
        Err(SecretKeyError::InvalidChar { .. })
    ));
}
