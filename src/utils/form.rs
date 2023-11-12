

/// Filters user input to make it safe to send to the database.
///
/// # Arguments
///
/// * `input` - The user input to filter.
///
/// # Returns
///
/// Returns the filtered user input.
pub fn filter_input(input: &str) -> String {
    // Your code to filter the user input goes here
    // Your code to filter the user input goes here
    // Implement the filtering logic
    let filtered_input = input.trim().to_string();

    // Return the filtered input
    filtered_input
}

/// Validates a string to only allow characters from a-z, A-Z, 0-9, and underscore (_).
///
/// # Arguments
///
/// * `input` - The string to validate.
///
/// # Returns
///
/// Returns `true` if the string is valid, `false` otherwise.
pub fn validate_string_alphanumeric_underscore(input: &str) -> bool {
    let valid_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_".chars().collect();
    
    for c in input.chars() {
        if !valid_chars.contains(&c) {
            return false;
        }
    }
    
    true
}

/// Validates a string to only allow alphabetic characters and spaces.
///
/// # Arguments
///
/// * `input` - The string to validate.
/// * `min_length` - The minimum length of the string (optional).
/// * `max_length` - The maximum length of the string (optional).
///
/// # Returns
///
/// Returns `true` if the string is valid, `false` otherwise.
pub fn validate_string_alpha_space(input: &str, min_length: Option<usize>, max_length: Option<usize>) -> bool {
    if let Some(min) = min_length {
        if input.len() < min {
            return false;
        }
    }
    
    if let Some(max) = max_length {
        if input.len() > max {
            return false;
        }
    }
    
    if input.trim().is_empty() {
        return false;
    }
    
    for c in input.chars() {
        if !c.is_alphabetic() && c != ' ' {
            return false;
        }
    }
    
    true
}
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

/// Hashes a password.
///
/// # Arguments
///
/// * `password` - The password to hash.
///
/// # Returns
///
/// Returns the hashed password if successful, `false` otherwise.
pub fn hash_password(password: &str) -> Result<String, bool> {

    // Check if the password is empty
    if password.is_empty() {
        return Err(false);
}

    // Hash the password using a secure hashing algorithm
    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|_| false)?;

    Ok(hashed_password)
}

