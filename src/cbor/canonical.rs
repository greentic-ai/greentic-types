//! Helpers for deterministic canonical CBOR, hashing, and Base32 IDs.
use alloc::{string::String, vec::Vec};
use core::cmp::Ordering;

use blake3;
use ciborium::{de::from_reader, ser::into_writer, value::Value};
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

/// Errors emitted while canonicalizing or hashing CBOR payloads.
#[derive(Debug, Error)]
pub enum CanonicalError {
    /// Decoding CBOR failed.
    #[error("CBOR decode failed: {0}")]
    Decode(String),
    /// Encoding CBOR failed.
    #[error("CBOR encode failed: {0}")]
    Encode(String),
    /// CBOR contains a non-string map key.
    #[error("CBOR map keys must be text strings")]
    NonStringMapKey,
    /// CBOR contains a floating-point value; floats are forbidden by default.
    #[error("CBOR floats are not allowed in canonical data")]
    FloatNotAllowed,
    /// CBOR contains a tagged value; tags are not allowed.
    #[error("CBOR tags are not allowed in canonical data")]
    TagNotAllowed,
    /// Base32 decode failed.
    #[error("Base32 decode failed: {0}")]
    Base32(#[from] Base32Error),
    /// CBOR payload is not canonical.
    #[error("CBOR payload is not canonical")]
    NotCanonical,
}

/// Result type returned by the canonical helpers.
pub type Result<T> = core::result::Result<T, CanonicalError>;

/// Encode a value as canonical CBOR bytes.
pub fn to_canonical_cbor<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut interim = Vec::new();
    into_writer(value, &mut interim).map_err(|err| CanonicalError::Encode(err.to_string()))?;
    canonicalize(&interim)
}

/// Deserialize CBOR ignoring canonical constraints.
pub fn from_cbor<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    from_reader(bytes).map_err(|err| CanonicalError::Decode(err.to_string()))
}

/// Ensure bytes are canonical by re-encoding and comparing.
pub fn ensure_canonical(bytes: &[u8]) -> Result<()> {
    let canonical = canonicalize(bytes)?;
    if bytes != canonical.as_slice() {
        return Err(CanonicalError::NotCanonical);
    }
    Ok(())
}

/// Parse CBOR and re-encode in canonical form.
pub fn canonicalize(bytes: &[u8]) -> Result<Vec<u8>> {
    let value: Value = from_reader(bytes).map_err(|err| CanonicalError::Decode(err.to_string()))?;
    let canonical = canonicalize_value(value)?;
    let mut buf = Vec::new();
    into_writer(&canonical, &mut buf).map_err(|err| CanonicalError::Encode(err.to_string()))?;
    Ok(buf)
}

fn canonicalize_value(value: Value) -> Result<Value> {
    match value {
        Value::Integer(_) | Value::Bytes(_) | Value::Text(_) | Value::Bool(_) | Value::Null => {
            Ok(value)
        }
        Value::Array(elements) => {
            let canonical_elements = elements
                .into_iter()
                .map(canonicalize_value)
                .collect::<Result<Vec<_>>>()?;
            Ok(Value::Array(canonical_elements))
        }
        Value::Map(entries) => {
            let mut canonical_entries = entries
                .into_iter()
                .map(|(key, val)| {
                    let canonical_key = match key {
                        Value::Text(text) => Value::Text(text),
                        _ => return Err(CanonicalError::NonStringMapKey),
                    };
                    let canonical_val = canonicalize_value(val)?;
                    Ok((canonical_key, canonical_val))
                })
                .collect::<Result<Vec<_>>>()?;
            canonical_entries.sort_unstable_by(|(a, _), (b, _)| compare_map_keys(a, b));
            Ok(Value::Map(canonical_entries))
        }
        Value::Float(_) => Err(CanonicalError::FloatNotAllowed),
        Value::Tag(_, _) => Err(CanonicalError::TagNotAllowed),
        _ => Err(CanonicalError::TagNotAllowed),
    }
}

fn compare_map_keys(a: &Value, b: &Value) -> Ordering {
    let (a_bytes, b_bytes) = match (a, b) {
        (Value::Text(a), Value::Text(b)) => (a.as_bytes(), b.as_bytes()),
        _ => panic!("map keys must be text strings"),
    };
    match a_bytes.len().cmp(&b_bytes.len()) {
        Ordering::Equal => a_bytes.cmp(b_bytes),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;

    #[test]
    fn canonicalize_reorders_map_keys() {
        let non_canonical = vec![
            0xA2, // map of two entries
            0x61, b'b', 0x01, // key "b" -> 1
            0x61, b'a', 0x02, // key "a" -> 2
        ];

        let canonical = match canonicalize(&non_canonical) {
            Ok(bytes) => bytes,
            Err(err) => panic!("canonicalize failed: {err:?}"),
        };
        assert_ne!(non_canonical, canonical);
        assert!(ensure_canonical(&canonical).is_ok());
    }

    #[test]
    fn ensure_canonical_rejects_indefinite_maps() {
        let indefinite = vec![
            0xBF, // start indefinite-length map
            0x61, b'a', 0x01, // "a": 1
            0xFF, // break
        ];
        assert!(matches!(
            ensure_canonical(&indefinite),
            Err(CanonicalError::NotCanonical)
        ));
    }

    #[test]
    fn base32_decode_accepts_lowercase() {
        let payload = [0x12, 0x34];
        let encoded = encode_base32_crockford(&payload);
        let lowercase = encoded.to_lowercase();
        let decoded = match decode_base32_crockford(&lowercase) {
            Ok(bytes) => bytes,
            Err(err) => panic!("decode failed: {err:?}"),
        };
        assert_eq!(decoded, payload.to_vec());
    }
}

/// Returns the first 128 bits of the Blake3 hash of the referenced bytes.
pub fn blake3_128(bytes: &[u8]) -> [u8; 16] {
    let mut out = [0u8; 16];
    out.copy_from_slice(&blake3::hash(bytes).as_bytes()[..16]);
    out
}

const BASE32_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// Encode bytes to Crockford Base32 (no padding, uppercase).
pub fn encode_base32_crockford(bytes: &[u8]) -> String {
    let mut bits = 0u32;
    let mut available = 0;
    let mut output = String::with_capacity((bytes.len() * 8).div_ceil(5));

    for &byte in bytes {
        bits = (bits << 8) | u32::from(byte);
        available += 8;
        while available >= 5 {
            available -= 5;
            let index = ((bits >> available) & 0x1f) as usize;
            output.push(BASE32_ALPHABET[index] as char);
        }
    }

    if available > 0 {
        let index = ((bits << (5 - available)) & 0x1f) as usize;
        output.push(BASE32_ALPHABET[index] as char);
    }

    output
}

/// Decode Crockford Base32, accepting lowercase input.
pub fn decode_base32_crockford(value: &str) -> core::result::Result<Vec<u8>, Base32Error> {
    let mut buffer = 0u32;
    let mut bits = 0;
    let mut output = Vec::with_capacity((value.len() * 5) / 8);

    for ch in value.chars() {
        let digit = match ch {
            '0'..='9' => ch,
            'a'..='z' => ch.to_ascii_uppercase(),
            'A'..='Z' => ch,
            other => return Err(Base32Error::InvalidCharacter(other)),
        };

        let val = match digit {
            'I' | 'L' => 1,
            'O' => 0,
            symbol => {
                let idx = BASE32_ALPHABET
                    .iter()
                    .position(|&b| b == symbol as u8)
                    .ok_or(Base32Error::InvalidCharacter(symbol))?;
                idx as u8
            }
        };

        buffer = (buffer << 5) | u32::from(val);
        bits += 5;

        if bits >= 8 {
            bits -= 8;
            output.push(((buffer >> bits) & 0xFF) as u8);
        }
    }

    if bits != 0 && (buffer & ((1 << bits) - 1)) != 0 {
        return Err(Base32Error::IncompleteByte);
    }

    Ok(output)
}

/// Crockford Base32 decode errors.
#[derive(Debug, Error)]
pub enum Base32Error {
    /// Character not part of the Crockford alphabet.
    #[error("invalid character {0}")]
    InvalidCharacter(char),
    /// Payload ended with leftover bits that are not zero.
    #[error("incomplete trailing bits")]
    IncompleteByte,
}
