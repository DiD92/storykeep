use std::hash::Hasher;
use twox_hash::XxHash64;

/// Computes the hexadecimal representation of the `bytes` slice.
///
/// The code used the `twox_hash::XxHash64` hasher to compute the
/// hash and the formats the result to a `String` representation.
pub fn compute_hash_from_bytes(bytes: &[u8]) -> String {
    let mut hasher = XxHash64::default();

    hasher.write(bytes);

    format!("{:x}", hasher.finish())
}

/// Computes the hexadecimal representation of the `text` slice.
///
/// The function calls `compute_hash_from_bytes` with the parameter
/// converted to a byte slice.
pub fn compute_hash_from_text(text: &str) -> String {
    compute_hash_from_bytes(text.as_bytes())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_consistent_hash_generation() {
        let text = "Hello World!";

        assert_eq!(compute_hash_from_text(text), compute_hash_from_text(text));
    }
}
