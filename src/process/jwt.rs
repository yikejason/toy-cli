use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const SECRET_KEY: &[u8] = b"secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    aud: String,
    exp: i64,
}

impl Claims {
    pub fn new(sub: String, aud: String, exp: i64) -> Self {
        Self { sub, aud, exp }
    }
}

pub fn process_jwt_sign(sub: String, aud: String, exp: i64) -> Result<String> {
    let jwt_claims = Claims::new(sub, aud, exp);
    let token = encode(
        &Header::default(),
        &jwt_claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )?;
    Ok(token)
}

pub fn process_jwt_verify(token: &str, aud: &str, sub: &str) -> Result<bool> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[aud.to_string()]);
    validation.sub = Some(sub.to_string());
    validation.set_required_spec_claims(&["exp", "aud", "sub"]);
    let data = decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &validation)?;
    println!("{:#?}", data);
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_sign_verify() {
        let token = process_jwt_sign("sub".into(), "aud".into(), 1731141312).unwrap();
        let valid = process_jwt_verify(&token, "aud", "sub").unwrap();
        assert!(valid);
    }
}
