use std::pin::Pin;

use crate::errors::user::UserError;
use crate::models::{jwt::UserToken};
use actix_web::{HttpRequest, FromRequest, dev::Payload};
use futures_util::Future;
use uuid::Uuid;


pub struct AuthenticatedUser {
    pub email: String,
    pub id: Uuid,
}

impl From<UserToken> for AuthenticatedUser {
    fn from(token: UserToken) -> Self {
        AuthenticatedUser { email: token.email, id: token.id }
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = UserError;
    type Future = Pin<Box<dyn Future<Output = Result<AuthenticatedUser, Self::Error>>>>;

    fn from_request(request: &HttpRequest, _pl: &mut Payload
    ) -> Self::Future {

        if let Ok(jwt) = UserToken::parse_jwt_from_request(request) {
            if let Ok(user_token) = UserToken::decode_token(&jwt) {
                return Box::pin(async move {
                    Ok(AuthenticatedUser::from(user_token))
                });
            }
        }

        Box::pin(async move {
            Err(UserError::Unauthorized)
        })
    }
}