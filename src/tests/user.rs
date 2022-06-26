#[cfg(test)]
mod tests {
    use crate::{api, db as database, models};
    use actix_web::{
        error,
        http::{self, header::ContentType},
        test, web, App, HttpResponse,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    const USERS_ROUTE: &str = "/api/users";
    const LOGIN_ROUTE: &str = "/api/login";

    #[actix_web::test]
    async fn test_create_user_success() {
        let mut app = test::init_service(App::new().configure(config_app)).await;

        let params = models::user::NewUser {
            name: "Test User".to_string(),
            email: "testunqiueuser@email.com".to_string(),
            password: "Pa$$w0rd".to_string(),
        };

        let res = test::call_service(
            &mut app,
            test::TestRequest::post()
                .set_json(&params)
                .uri(USERS_ROUTE)
                .to_request(),
        )
        .await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn test_create_user_failure() {
        let mut app = test::init_service(App::new().configure(config_app)).await;

        let params = models::user::NewUser {
            name: "Test User".to_string(),
            email: "testuser@".to_string(), // incorrect email format
            password: "Pa$$w0rd".to_string(),
        };

        let res = test::call_service(
            &mut app,
            test::TestRequest::post()
                .set_json(&params)
                .uri(USERS_ROUTE)
                .to_request(),
        )
        .await;

        assert!(res.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_get_users_success() {
        let mut app = test::init_service(App::new().configure(config_app)).await;

        let params = models::user::NewUser {
            name: "Test User".to_string(),
            email: "testuser@email.com".to_string(), // incorrect email format
            password: "Pa$$w0rd".to_string(),
        };

        let _res = test::call_service(
            &mut app,
            test::TestRequest::post()
                .set_json(&params)
                .uri(USERS_ROUTE)
                .to_request(),
        )
        .await;

        let params = models::user::LoginData {
            email: "testuser@email.com".to_string(), // incorrect email format
            password: "Pa$$w0rd".to_string(),
        };

        let response = test::call_service(
            &mut app,
            test::TestRequest::post()
                .set_json(&params)
                .uri(LOGIN_ROUTE)
                .to_request(),
        )
        .await;

        let body: TokenResponse = test::read_body_json(response).await;

        let token = format!("{} {}", body.token_type, body.token);

        let res = test::call_service(
            &mut app,
            test::TestRequest::get()
                .insert_header((
                    http::header::AUTHORIZATION,
                    http::header::HeaderValue::from_str(&token).unwrap(),
                ))
                .uri(USERS_ROUTE)
                .to_request(),
        )
        .await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_users_unauthorized() {
        let mut app = test::init_service(App::new().configure(config_app)).await;

        let res = test::call_service(
            &mut app,
            test::TestRequest::get().uri(USERS_ROUTE).to_request(),
        )
        .await;

        assert!(res.status().is_client_error());
    }

    fn config_app(cfg: &mut web::ServiceConfig) {
        let json_cfg = web::JsonConfig::default().error_handler(|err, _| {
            error::InternalError::from_response(
                "",
                HttpResponse::BadRequest()
                    .content_type(ContentType::json())
                    .json(json!({
                        "error": err.to_string()
                    })),
            )
            .into()
        });

        let db_pool = database::init_database(&"TEST_DATABASE_URL".to_string());

        cfg.app_data(json_cfg.clone())
            .app_data(web::Data::new(db_pool.clone()))
            .configure(api::users_controller::register_routes);
    }

    #[derive(Serialize, Deserialize)]
    struct TokenResponse {
        token: String,
        token_type: String,
    }

}
