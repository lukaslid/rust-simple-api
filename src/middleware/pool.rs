use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::{Method},
    body::EitherBody,
    Error, HttpResponse,
};

use futures_util::future::LocalBoxFuture;
// use futures::{
    // future::{ok, Ready},
    // Future,
// };

use std::future::{Ready, Future, ready};


use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::{models::jwt::UserToken, errors::user::UserError, middleware::auth};

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    // type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, mut request: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = false;

        if Method::OPTIONS == *request.method() {
            authenticate_pass = true;
        }
        if let Some(authen_header) = request.headers().get("Authorization") {
            if let Ok(auth_str) = authen_header.to_str() {
                println!("AUTH ASTR {}", &auth_str);
                if !(auth_str.starts_with("bearer") || auth_str.starts_with("Bearer")) {
                    authenticate_pass = false;
                }

                let token = auth_str[6..auth_str.len()].trim();
                println!("tokennn {}", &token);

                let tt = UserToken::parse_token(&token.to_string()).unwrap();
                println!("ttttt {}", &tt.id);
                match UserToken::parse_token(&token.to_string()) {
                    Ok(token) => {
                        // println!("tokennnnnnnn {}", &token);
                        authenticate_pass = token.validate_token();
                        println!("pass {}", &authenticate_pass);
                    },
                    Err(_) => authenticate_pass = false
                }
            }


        }
        if authenticate_pass {
            let res = self.service.call(request);
            
            Box::pin(async move {
                res.await.map(ServiceResponse::map_into_left_body)
            })
        } else {
            let (request, _pl) = request.into_parts();
            let response = HttpResponse::Unauthorized()
                .finish()
                .map_into_right_body();

            Box::pin(async { Ok(ServiceResponse::new(request, response)) })
        }
    }

    forward_ready!(service);
}