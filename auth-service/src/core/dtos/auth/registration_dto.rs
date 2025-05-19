use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct RegistrationDto {
    #[validate(length(min = 2, message = "Name must be greater than 2 chars"))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(
        custom(
            function = "crate::utils::util::validate_password"
        )
    )]
    pub password: String,
}