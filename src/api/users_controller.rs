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
        Ok(_) => (),
        Err(e) => return Err(UserError::from(e)),
    };


    let user = web::block(move || User::create(&conn, new_user))
        .await
        .unwrap()?;

    Ok(web::Json(user))
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


// #[cfg(test)]
// mod tests {
//     use super::*;

//     use actix_cors::Cors;
//     use actix_web::{http, http::StatusCode, test, App};
//     use futures::FutureExt;
//     use http::header;

//     #[actix_web::test]
//     async fn test_create() {
//         let pool = config::db::migrate_and_config_db(":memory:");

//         let mut app = test::init_service(
//             App::new()
//                 .wrap(
//                     Cors::default()
//                         .send_wildcard()
//                         .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
//                         .allowed_header(http::header::CONTENT_TYPE)
//                         .max_age(3600),
//                 )
//                 .app_data(web::Data::new(pool.clone()))
//                 .wrap(actix_web::middleware::Logger::default())
//                 .configure(register_routes),
//         )
//         .await;

//         let resp = test::TestRequest::post()
//             .uri("/api/create")
//             .set(header::ContentType::json())
//             .set_payload(
//                 r#"{"username":"admin","email":"admin@gmail.com","password":"123456"}"#.as_bytes(),
//             )
//             .send_request(&mut app)
//             .await;

//     // let data = test::read_body(resp).await;

//     // println!("{:#?}", &data);
//     assert_eq!(resp.status(), StatusCode::OK);
        
//     }
// }