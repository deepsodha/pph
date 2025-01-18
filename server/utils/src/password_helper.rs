use bcrypt::{hash, verify};
// use rand::Rng;
// Function to hash a password using bcrypt
// Function to hash a password using bcrypt with a reduced cost
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    // Hash the password using bcrypt with a reduced cost factor
    let hashed_password = hash(password, 8)?; // Experiment with different cost values

    // Return the hashed password
    Ok(hashed_password)
}

// Function to verify a password against a hashed value using bcrypt
pub fn verify_password(hash: &str, password: &str) -> Result<bool, bcrypt::BcryptError> {
    // Verify the password against the provided hash
    verify(password, hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn password_hashing() {
        let hash = hash_password("password").unwrap();
        let is_valid = verify_password(&hash, "password").unwrap_or(false);
        assert!(is_valid);
        assert_ne!(&hash, &"password");
    }
}
