use strum::Display;

#[derive(Debug, Display)]
pub enum RoleEnum {
    #[strum(serialize = "admin")]
    Admin,
    #[strum(serialize = "user")]
    User,
}