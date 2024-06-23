use anyhow::anyhow;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_captcha_token(secret: &str) -> Result<String> {
    let exp = Utc::now() + Duration::hours(2);

    let claims = Claims {
        sub: "x-client-login".to_string(),
        exp: exp.timestamp() as usize,
    };

    let Ok(token) = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) else {
        return Err(anyhow!("Error creating token"));
    };

    let new_token = format!("x-client-login:{}", token);
    Ok(new_token)
}

pub fn verify_captcha_token(token: &str, secret: &str) -> Result<String> {
    let raw_token = token.replace("x-client-login:", "");
    let Ok(decoded) = decode::<Claims>(
        &raw_token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ) else {
        return Err(anyhow!("Error decoding token"));
    };

    if decoded.claims.sub.len() == 0 {
        return Err(anyhow!("Invalid token"));
    }

    Ok(decoded.claims.sub)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_captcha_token() {
        // Generate token
        let token = create_captcha_token("secret").unwrap();
        assert!(token.len() > 0);
        assert!(token.starts_with("x-client-login:"));

        // Validate claims
        let sub = verify_captcha_token(&token, "secret").unwrap();
        assert_eq!(sub, "x-client-login");
    }
}
