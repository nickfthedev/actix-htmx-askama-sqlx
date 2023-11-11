

/// Verifies an email address.
///
/// # Arguments
///
/// * `email` - The email address to verify.
///
/// # Returns
///
/// Returns `true` if the email address is valid, `false` otherwise.
pub fn verify_email(email: &str) -> bool {
    // Your code to verify the email address goes here
    // Your code to verify the email address goes here
    // Check if the email address is empty
    if email.is_empty() {
        return false;
    }

    // Check if the email address contains an '@' symbol
    if !email.contains('@') {
        return false;
    }

    // Check if the email address contains a domain
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || parts[1].is_empty() {
        return false;
    }

    // Check if the domain contains a dot
    if !parts[1].contains('.') {
        return false;
    }

    // Check if the domain has at least two characters after the dot
    let domain_parts: Vec<&str> = parts[1].split('.').collect();
    if domain_parts.len() < 2 || domain_parts[1].is_empty() {
        return false;
    }

    // All checks passed, the email address is valid
    true
}
