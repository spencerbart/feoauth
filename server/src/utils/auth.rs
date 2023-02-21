use std::env;

use http::HeaderValue;
use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn get_user_id(auth_header: Option<&HeaderValue>) -> Option<String> {
    let auth_header = match auth_header {
        Some(auth_header) => auth_header,
        None => return None,
    };

    let jwt = match auth_header.to_str() {
        Ok(jwt) => jwt.split(" ").collect::<Vec<&str>>().get(1)?.to_string(),
        Err(_) => return None,
    };

    let claims = auth_jwt(&jwt)?;

    match Uuid::parse_str(claims.sub.as_str()) {
        Ok(_) => return Some(claims.sub),
        Err(_) => return None,
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

fn auth_jwt(jwt: &str) -> Option<Claims> {
    let key = env::var("JWT_SECRET").unwrap();

    let validation = Validation::new(Algorithm::HS256);
    let token_data =
        match decode::<Claims>(jwt, &DecodingKey::from_secret(key.as_ref()), &validation) {
            Ok(c) => c,
            Err(err) => {
                match *err.kind() {
                    ErrorKind::InvalidToken => return None, // Example on how to handle a specific error
                    ErrorKind::InvalidIssuer => return None, // Example on how to handle a specific error
                    _ => return None,
                }
            }
        };
    Some(token_data.claims)
}
