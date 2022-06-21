use super::core::*;
use async_trait::async_trait;
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AuthService {
    secret: String,
    pwd: String,
}

impl AuthService {
    pub fn new() -> Result<AuthService, AdminErr> {
        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| AdminErr::ConfNone("JWT_SECRET".to_string()))?;
        let pwd = std::env::var("PWD")
            .map_err(|_| AdminErr::ConfNone("PWD".to_string()))?;
        Ok(AuthService { pwd, secret })
    }
}

#[async_trait]
impl Auth for AuthService {
    async fn get_token(&self, pwd: &str) -> Result<String, AdminErr> {
        if pwd != self.pwd {
            return Err(AdminErr::InvdCreds);
        }
        return Ok(get_default_token(&self.secret, 3600));
    }
    async fn validate_token(&self, token: &str) -> bool {
        decode_default_token(token, &self.secret).is_ok()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    exp: usize,
}

fn get_default_token(secret: &str, exp_offset: u64) -> String {
    let elasped = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time travel")
        .as_secs();

    encode(
        &Header::default(),
        &Claims {
            exp: (elasped + exp_offset) as usize,
        },
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

fn decode_default_token(token: &str, secret: &str) -> Result<(), ()> {
    let val = Validation::default();
    let result = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &val,
    );
    result.map(|_| ()).map_err(|_| ())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_auth() {
        let secret = "12345678";
        let token = get_default_token(&secret, 3600);
        let result = decode_default_token(&token, &secret);
        assert!(result.is_ok())
    }

    #[test]
    fn should_not_auth() {
        let secret = "12345678";
        let token = get_default_token(&secret, 60);
        let result = decode_default_token(&token, "1234");
        assert!(result.is_err())
    }
}
