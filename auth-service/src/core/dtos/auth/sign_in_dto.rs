use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct SignInDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(
        custom(
            function = "crate::utils::util::validate_password"
        )
    )]
    pub password: String,
}