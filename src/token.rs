use jwt_simple::prelude::*;

use crate::error::Result;

pub fn create_captcha_token(secret: &str) -> Result<String> {
    let key = HS256Key::from_bytes(secret.as_bytes());
    let claims = Claims::create(Duration::from_hours(1)).with_subject("x-client-login");
    key.authenticate(claims)
}
