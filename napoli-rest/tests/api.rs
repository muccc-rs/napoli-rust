use axum::body::Body;
use http::Request;
use http_body_util::BodyExt;
use napoli_rest::routes::AppState;
use napoli_server_migrations::{Migrator, MigratorTrait};
use sea_orm::ConnectOptions;
use tower::ServiceExt;

async fn setup() -> axum::Router {
    setup_with_api_key(None).await
}

async fn setup_with_api_key(api_key: Option<String>) -> axum::Router {
    let mut opts = ConnectOptions::new("sqlite::memory:".to_string());
    opts.max_connections(1);
    let db = sea_orm::Database::connect(opts).await.unwrap();
    Migrator::up(&db, None).await.unwrap();
    napoli_rest::build_app_with_api_key(AppState { db }, api_key)
}

async fn body_json(response: axum::response::Response) -> serde_json::Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn get_orders_empty() {
    let app = setup().await;

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/orders")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let json = body_json(resp).await;
    assert_eq!(json, serde_json::json!([]));
}

#[tokio::test]
async fn create_order() {
    let app = setup().await;

    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/orders")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"menu_url": "https://pizza.example.com"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let json = body_json(resp).await;
    assert_eq!(json["id"], 1);
    assert_eq!(json["menu_url"], "https://pizza.example.com");
    assert_eq!(json["state"], "OPEN");
    assert_eq!(json["entries"], serde_json::json!([]));
}

#[tokio::test]
async fn create_and_get_order() {
    let app = setup().await;

    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/orders")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"menu_url": "https://example.com"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/orders/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let json = body_json(resp).await;
    assert_eq!(json["id"], 1);
    assert_eq!(json["menu_url"], "https://example.com");
}

#[tokio::test]
async fn get_order_not_found() {
    let app = setup().await;

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/orders/999")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn add_entry_to_order() {
    let app = setup().await;

    // Create order
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/orders")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"menu_url": "https://example.com"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Add entry
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/orders/1/entries")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "buyer": "Max",
                        "food": "Margherita",
                        "price": "8.50"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let json = body_json(resp).await;
    assert_eq!(json["entries"].as_array().unwrap().len(), 1);
    assert_eq!(json["entries"][0]["buyer"], "Max");
    assert_eq!(json["entries"][0]["food"], "Margherita");
    assert_eq!(json["entries"][0]["paid"], false);
}

#[tokio::test]
async fn add_entry_order_not_found() {
    let app = setup().await;

    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/orders/999/entries")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "buyer": "Max",
                        "food": "Margherita",
                        "price": "8.50"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn add_entry_invalid_price() {
    let app = setup().await;

    // Create order
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/orders")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"menu_url": "https://example.com"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Bad price
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/orders/1/entries")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "buyer": "Max",
                        "food": "Margherita",
                        "price": "not-a-number"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn get_orders_summary() {
    let app = setup().await;

    // Create order with entry
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/orders")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"menu_url": "https://example.com"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/orders/1/entries")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "buyer": "Max",
                        "food": "Margherita",
                        "price": "8.50"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Summary mode — no entries field
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/orders?summary=true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let json = body_json(resp).await;
    let orders = json.as_array().unwrap();
    assert_eq!(orders.len(), 1);
    assert!(orders[0].get("entries").is_none());

    // Full mode — has entries
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/orders")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let json = body_json(resp).await;
    let orders = json.as_array().unwrap();
    assert_eq!(orders[0]["entries"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn create_order_menu_url_too_long() {
    let app = setup().await;

    let long_url = "x".repeat(300);
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/orders")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"menu_url": long_url}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn add_entry_price_too_expensive() {
    let app = setup().await;

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/orders")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"menu_url": "https://example.com"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/orders/1/entries")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "buyer": "Max",
                        "food": "Gold Pizza",
                        "price": "999999999.99"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn api_key_rejects_without_token() {
    let app = setup_with_api_key(Some("secret123".into())).await;

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/orders")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 401);
}

#[tokio::test]
async fn api_key_rejects_wrong_token() {
    let app = setup_with_api_key(Some("secret123".into())).await;

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/orders")
                .header("authorization", "Bearer wrongkey")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 401);
}

#[tokio::test]
async fn api_key_accepts_correct_token() {
    let app = setup_with_api_key(Some("secret123".into())).await;

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/orders")
                .header("authorization", "Bearer secret123")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn no_api_key_config_allows_all() {
    let app = setup_with_api_key(None).await;

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/orders")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}
