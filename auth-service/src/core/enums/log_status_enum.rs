use strum::Display;

#[derive(Debug, Display)]
pub enum LogStatusEnum {
    #[strum(serialize = "error")]
    Error,

    #[strum(serialize = "info")]
    Info,

    #[strum(serialize = "verbose")]
    Verbose,

    #[strum(serialize = "warning")]
    Warning,
}