use crate::models::jwt::UserToken;
use crate::models::user::{User, LoginData, NewUser};
use crate::models::auth::AuthenticatedUser;
use crate::errors::user::UserError;
use crate::db::PostgresPool;
use actix_web::{get, post, web, Responder};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

#[get("/api/users")]
async fn find_all(pool: web::Data<PostgresPool>, _user: AuthenticatedUser) -> Result<impl Responder, UserError> {
    let conn = pool.get()
        .or_else(|_e| return Err(UserError::InternalError))
        .unwrap();
    
    let users = web::block(move || User::get_all(&conn))
        .await
        .unwrap()?;
        
    Ok(web::Json(users))
}

#[get("/api/users/{id}")]
async fn find(pool: web::Data<PostgresPool>, id: web::Path<Uuid>, _user: AuthenticatedUser) -> Result<impl Responder, UserError> {
    let conn = pool.get()
        .or_else(|_e| return Err(UserError::InternalError))
        .unwrap();
    
    let user = web::block(move ||User::get(&conn, id.into_inner()))
        .await
        .unwrap()?;
    Ok(web::Json(user))
}

#[post("/api/users")]
async fn create(pool: web::Data<PostgresPool>, new_user: web::Json<NewUser>) -> Result<impl Responder, UserError> {
    let conn = pool.get()
        .or_else(|_e| return Err(UserError::InternalError))
        .unwrap();
    
    let new_user = new_user.into_inner();

    match new_user.validate() {
        Ok(_) => {
            let user = web::block(move || User::create(&conn, new_user))
                .await
                .unwrap()?;
            Ok(web::Json(user))
        },
        Err(e) => Err(UserError::from(e)),
    }
}


#[post("/api/login")]
async fn login(pool: web::Data<PostgresPool>, login_data: web::Json<LoginData>) -> Result<impl Responder, UserError> {
    let conn = pool.get()
        .or_else(|_e| return Err(UserError::InternalError))
        .unwrap();

    
    let logged_user = web::block(move || User::login(&conn, login_data.into_inner()))
        .await.unwrap()?;

    match UserToken::generate_token(&logged_user.id, &logged_user.email) {
        Ok(token) => {
            let json_data = json!({ "token": token, "token_type": "bearer" });
            Ok(web::Json(json_data))
        },
        Err(e) => Err(e)
    }
}

#[post("/api/refresh-token")]
async fn refresh_token(user_token: UserToken) -> Result<impl Responder, UserError> {
    match UserToken::generate_token(&user_token.id, &user_token.email) {
        Ok(token) => {
            let json_data = json!({ "token": token, "token_type": "bearer" });
            Ok(web::Json(json_data))
        },
        Err(e) => Err(e)
    }
}


pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(login);
    cfg.service(refresh_token);
}