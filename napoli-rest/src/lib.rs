pub mod errors;
pub mod models;
pub mod routes;

use axum::extract::Request;
use axum::middleware::{self, Next};
use axum::response::IntoResponse;
use routes::AppState;
use tower_http::cors;

pub fn build_app(state: AppState) -> axum::Router {
    build_app_with_api_key(state, std::env::var("API_KEY").ok())
}

pub fn build_app_with_api_key(state: AppState, api_key: Option<String>) -> axum::Router {
    let cors = cors::CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods([http::Method::GET, http::Method::POST])
        .allow_origin(cors::Any);

    let mut app = axum::Router::new()
        .route("/orders", axum::routing::get(routes::get_orders))
        .route("/orders", axum::routing::post(routes::create_order))
        .route("/orders/:id", axum::routing::get(routes::get_order))
        .route(
            "/orders/:id/entries",
            axum::routing::post(routes::add_entry),
        );

    if let Some(expected_key) = api_key {
        app = app.layer(middleware::from_fn(move |req: Request, next: Next| {
            let expected_key = expected_key.clone();
            async move {
                let authorized = req
                    .headers()
                    .get("authorization")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.strip_prefix("Bearer "))
                    .map(|token| token == expected_key)
                    .unwrap_or(false);

                if authorized {
                    next.run(req).await
                } else {
                    (
                        http::StatusCode::UNAUTHORIZED,
                        axum::Json(serde_json::json!({"error": "unauthorized"})),
                    )
                        .into_response()
                }
            }
        }));
    }

    app.layer(cors).with_state(state)
}
