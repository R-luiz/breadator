//! Calculator state in URLs: `?s=` holds the engine input as base64url (no padding) JSON, so any
//! result can be shared, bookmarked and rendered by the server without JavaScript.

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Longest accepted `s` parameter; a real formula is a few hundred bytes.
const MAX_LEN: usize = 16 * 1024;

pub fn encode<T: Serialize>(value: &T) -> String {
    URL_SAFE_NO_PAD.encode(serde_json::to_vec(value).unwrap_or_default())
}

pub fn decode<T: DeserializeOwned>(s: &str) -> Option<T> {
    if s.len() > MAX_LEN {
        return None;
    }
    let bytes = URL_SAFE_NO_PAD.decode(s.trim_end_matches('=')).ok()?;
    serde_json::from_slice(&bytes).ok()
}

/// Lenient number parsing for form fields: empty → `None`, comma or dot decimals.
pub fn number(field: Option<&str>) -> Option<f64> {
    let s = field?.trim();
    if s.is_empty() {
        return None;
    }
    s.replace(',', ".").parse::<f64>().ok().filter(|v| v.is_finite())
}
