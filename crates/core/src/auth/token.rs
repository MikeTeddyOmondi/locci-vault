pub struct TokenStore;

/// Generates a token.
///
/// TODO: [prod] this should generate a secure token (e.g., JWT).
pub fn generate_token() -> String {
    "valid".to_string()
}
