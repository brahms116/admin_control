use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    exp: usize,
}

pub fn get_default_token(secret: &str, exp_offset: u64) -> String {
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

pub fn decode_default_token(token: &str, secret: &str) -> Result<(), ()> {
    let val = Validation::default();
    let result = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &val);
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
