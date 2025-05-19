use std::borrow::Cow;
use validator::ValidationError;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::config::application_config::PASSWORD_MIN_LENGTH;

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let mut has_upper: bool = false;
    let mut has_lower: bool = false;
    let mut has_digit: bool = false;
    let mut has_special: bool = false;

    for char in password.chars() {
        if char.is_uppercase() { has_upper = true; }
        if char.is_lowercase() { has_lower = true; }
        if char.is_digit(10) { has_digit = true; }
        if "!@#$%^&*()_+-=[]{}|;:'\",.<>/?".contains(char) { has_special = true; }
    }

    if !has_upper {
        return Err(ValidationError::new("password").with_message(Cow::Borrowed("password must contain at least one uppercase letter")));
    }

    if !has_lower {
        return Err(ValidationError::new("password").with_message(Cow::Borrowed("password must contain at least one lowercase letter")));
    }

    if !has_digit {
        return Err(ValidationError::new("password").with_message(Cow::Borrowed("password must contain at least one digit")));
    }

    if !has_special {
        return Err(ValidationError::new("password").with_message(Cow::Borrowed("must contain at least one special character")));
    }

    if password.len() < PASSWORD_MIN_LENGTH {
        return Err(ValidationError::new("password must be at least 8 characters long").with_message(Cow::Borrowed("password must be at least 8 characters long")));
    }

    Ok(())
}

pub fn hash_password(password: String) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    match verify(password, hashed) {
        Ok(result) => result,
        Err(_) => false,
    }
}