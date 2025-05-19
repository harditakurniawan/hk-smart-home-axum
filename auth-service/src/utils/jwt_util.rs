use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::repositories::user_repository::UserWithRole;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct IAuth {
    iat: u64,
    pub id: i64,
    pub email: String,
    pub role: String,
    exp: u64,
}

pub fn generate_token(user: &UserWithRole, private_key: String) -> String {
    let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap();
    let header = Header::new(Algorithm::RS256);

    let exp: chrono::DateTime<Utc> = Utc::now() + Duration::minutes(15);
    
    let claims = IAuth {
        iat: Utc::now().timestamp() as u64,
        id: user.id as i64,
        email: user.email.clone(),
        role: user.role_name.clone(),
        exp: exp.timestamp() as u64,
    };

    return encode::<IAuth>(&header, &claims, &encoding_key).unwrap();
}

pub fn decode_token(token: &str, public_key: String) -> Result<IAuth, errors::Error> {
    let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes())?;
    let mut validation = Validation::new(Algorithm::RS256);

    validation.validate_exp = true;
    validation.validate_aud = false;

    return decode::<IAuth>(token, &decoding_key, &validation).map(|data| data.claims);
}