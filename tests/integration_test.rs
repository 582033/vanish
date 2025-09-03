use actix_web::{test, web, App, http::StatusCode};
use serde_json::json;
use std::time::{Duration, SystemTime};
use vanish::{AppState, create_secret, get_secret, Secret};

// Helper to set up the application for testing
async fn setup_app() -> impl actix_web::dev::Service<actix_http::Request, Response = actix_web::dev::ServiceResponse, Error = actix_web::Error> {
    let app_data = web::Data::new(AppState {
        secrets: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
    });
    test::init_service(
        App::new()
            .app_data(app_data.clone())
            .service(create_secret)
            .service(get_secret),
    )
    .await
}

#[actix_web::test]
async fn test_create_and_get_secret_happy_path() {
    let app = setup_app().await;
    let secret_message = "This is a valid secret.";

    // 1. Create secret with specific expiration
    let req = test::TestRequest::post()
        .uri("/api/secrets")
        .set_json(&json!({ "message": secret_message, "expires_in_secs": 60 }))
        .to_request();
    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let secret_id = resp["id"].as_str().expect("Should have an id");

    // 2. Get secret
    let req = test::TestRequest::get()
        .uri(&format!("/api/secrets/{}", secret_id))
        .to_request();
    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp["message"], secret_message);

    // 3. Get again fails (burn after reading)
    let req = test::TestRequest::get()
        .uri(&format!("/api/secrets/{}", secret_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_create_with_default_expiration() {
    let app = setup_app().await;
    let secret_message = "Default expiration test.";

    // Create secret without specifying expiration
    let req = test::TestRequest::post()
        .uri("/api/secrets")
        .set_json(&json!({ "message": secret_message }))
        .to_request();
    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert!(resp["id"].as_str().is_some());
}

#[actix_web::test]
async fn test_invalid_expiration_value() {
    let app = setup_app().await;
    let req = test::TestRequest::post()
        .uri("/api/secrets")
        .set_json(&json!({ "message": "test", "expires_in_secs": 99999 }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}


#[actix_web::test]
async fn test_message_length_limit() {
    let app = setup_app().await;
    let long_message = "a".repeat(501);

    let req = test::TestRequest::post()
        .uri("/api/secrets")
        .set_json(&json!({ "message": long_message }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_secret_expiration() {
    let app_data = web::Data::new(AppState {
        secrets: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
    });
    let app = test::init_service(
        App::new()
            .app_data(app_data.clone())
            .service(create_secret)
            .service(get_secret),
    )
    .await;

    let secret_id = "expired_secret";
    let expired_secret = Secret {
        message: "This secret has expired.".to_string(),
        expires_at: SystemTime::now() - Duration::from_secs(1), // 1 second in the past
    };

    let mut secrets = app_data.secrets.lock().unwrap();
    secrets.insert(secret_id.to_string(), expired_secret);
    drop(secrets);

    let req = test::TestRequest::get()
        .uri(&format!("/api/secrets/{}", secret_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    
    let secrets = app_data.secrets.lock().unwrap();
    assert!(!secrets.contains_key(secret_id));
}
