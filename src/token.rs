use jwt_simple::prelude::*;

use crate::error::Result;

pub fn create_captcha_token(secret: &str) -> Result<String> {
    // For some reason, there are some miliseconds of drift between the server and the client
    // Let's just add a 10 second day to fix the issue
    let now = Clock::now_since_epoch();
    let drift = Duration::from_secs(10);

    let key = HS256Key::from_bytes(secret.as_bytes());
    let claims = Claims::create(Duration::from_hours(2))
        .invalid_before(now - drift)
        .with_subject("x-client-login");
    let token = key.authenticate(claims)?;
    Ok(format!("x-client-login:{}", token))
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
        let key = HS256Key::from_bytes("secret".as_bytes());
        let raw_token = token.replace("x-client-login:", "");
        let claims = key.verify_token::<NoCustomClaims>(&raw_token, None);
        assert!(claims.is_ok());
    }
}
