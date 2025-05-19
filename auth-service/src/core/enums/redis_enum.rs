use strum::Display;

#[derive(Debug, Display)]
pub enum RedisKey {
    #[strum(serialize = "hk-smart_home-systemconfig")]
    HK_SMART_HOME_SYSTEMCONFIG,
}