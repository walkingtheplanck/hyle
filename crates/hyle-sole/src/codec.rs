use crate::SoleModule;

/// Encodes a `.sole` module as pretty `.sole.json`.
///
/// # Errors
///
/// Returns a JSON serialization error if any contained value cannot be
/// represented as JSON.
pub fn encode_sole_json(module: &SoleModule) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(module)
}

/// Decodes a `.sole.json` string into a `.sole` module.
///
/// # Errors
///
/// Returns a JSON deserialization error if `source` is not valid `.sole.json`.
pub fn decode_sole_json(source: &str) -> Result<SoleModule, serde_json::Error> {
    serde_json::from_str(source)
}

/// Encodes a `.sole` module as pretty UTF-8 `.sole.json` bytes.
///
/// # Errors
///
/// Returns a JSON serialization error if any contained value cannot be
/// represented as JSON.
pub fn encode_sole_json_bytes(module: &SoleModule) -> Result<Vec<u8>, serde_json::Error> {
    encode_sole_json(module).map(String::into_bytes)
}

/// Decodes UTF-8 `.sole.json` bytes into a `.sole` module.
///
/// # Errors
///
/// Returns a JSON deserialization error if `source` is not valid `.sole.json`.
pub fn decode_sole_json_bytes(source: &[u8]) -> Result<SoleModule, serde_json::Error> {
    serde_json::from_slice(source)
}
