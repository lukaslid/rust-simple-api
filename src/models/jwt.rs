use std::pin::Pin;

use actix_web::{HttpRequest, FromRequest, dev::Payload};
use chrono::Utc;
use futures_util::Future;
use uuid::Uuid;
use jsonwebtoken::{EncodingKey, Header, Validation, Algorithm, DecodingKey};
use serde::{Serialize, Deserialize};

use crate::{errors::user::UserError};

// TODO: load enocding and decoding keys once
pub static KEY: [u8; 16] = *include_bytes!("../secret.key");

static ONE_HOUR: i64 = 60 * 1; // in seconds

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub email: String,
    pub id: Uuid
}

impl UserToken {
    pub fn generate_token(user_id: &Uuid, email: &String) -> Result<String, UserError> {
        let now = Utc::now().timestamp();
        let payload = UserToken {
            iat: now,
            exp: now + ONE_HOUR,
            id: user_id.clone(),
            email: email.clone()
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&KEY),
        )
        .map_err(|_e| UserError::InternalError)
    }

    pub fn decode_token(jwt: &String) -> Result<Self, UserError> {
        jsonwebtoken::decode::<UserToken>(
            jwt, 
            &DecodingKey::from_secret(&KEY), 
            &Validation::new(Algorithm::HS256)
        )
        .and_then(|t| Ok(t.claims))
        .map_err(|_e| UserError::Unauthorized)
    }

    pub fn verify_token(jwt: &String) -> Result<Self, UserError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = false;
        
        jsonwebtoken::decode::<UserToken>(
            jwt, 
            &DecodingKey::from_secret(&KEY), 
            &validation
        )
        .and_then(|t| Ok(t.claims))
        .map_err(|_e| UserError::Unauthorized)
    }

    pub fn parse_jwt_from_request(request: &HttpRequest) -> Result<String, UserError> {
        if let Some(authen_header) = request.headers().get("Authorization") {
            if let Ok(auth_str) = authen_header.to_str() {
                println!("header zz");
                if !(auth_str.starts_with("bearer") || auth_str.starts_with("Bearer")) {
                    return Err(UserError::Unauthorized);
                }
    
                let bearer_token: Vec<&str> = auth_str
                    .split_whitespace()
                    .collect();
    
                if bearer_token.len() != 2 {
                    return Err(UserError::Unauthorized);
                }
    
                let token = bearer_token[1];

                return Ok(token.to_owned());
            }
        }   
        Err(UserError::Unauthorized)
    }
}


impl FromRequest for UserToken {
    type Error = UserError;
    type Future = Pin<Box<dyn Future<Output = Result<UserToken, Self::Error>>>>;

    fn from_request(request: &HttpRequest, _pl: &mut Payload
    ) -> Self::Future {

        if let Ok(jwt) = UserToken::parse_jwt_from_request(request) {
            if let Ok(user_token) = UserToken::verify_token(&jwt) {
                println!("TOKEN {}", user_token.id);
                return Box::pin(async move {
                    Ok(user_token)
                });
            }
        }
        
        Box::pin(async move {
            Err(UserError::Unauthorized)
        })
    }
}