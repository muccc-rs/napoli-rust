use axum::extract::{Path, Query, State};
use axum::Json;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder as _};
use sea_orm::ActiveValue::{NotSet, Set};
use time::format_description::well_known::Rfc3339;

use napoli_server_persistent_entities::{order, order_entry};

use crate::errors::{self, ApiError};
use crate::models;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn get_orders(
    State(state): State<AppState>,
    Query(query): Query<models::OrdersQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if query.summary == Some(true) {
        let orders = order::Entity::find()
            .order_by(order::Column::Id, sea_orm::Order::Desc)
            .all(&state.db)
            .await
            .map_err(|e| errors::internal(e.to_string()))?;

        let orders: Vec<_> = orders.into_iter().map(models::order_summary).collect();
        Ok(Json(serde_json::to_value(orders).unwrap()))
    } else {
        let orders = order::Entity::find()
            .order_by(order::Column::Id, sea_orm::Order::Desc)
            .find_with_related(order_entry::Entity)
            .all(&state.db)
            .await
            .map_err(|e| errors::internal(e.to_string()))?;

        let orders: Vec<_> = orders
            .into_iter()
            .map(|(order, entries)| models::order_with_entries(order, entries))
            .collect();
        Ok(Json(serde_json::to_value(orders).unwrap()))
    }
}

pub async fn get_order(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<models::OrderResponse>, ApiError> {
    let orders = order::Entity::find_by_id(id)
        .find_with_related(order_entry::Entity)
        .all(&state.db)
        .await
        .map_err(|e| errors::internal(e.to_string()))?;

    let (order, entries) = orders
        .into_iter()
        .next()
        .ok_or_else(|| errors::not_found("order not found"))?;

    Ok(Json(models::order_with_entries(order, entries)))
}

pub async fn create_order(
    State(state): State<AppState>,
    Json(req): Json<models::CreateOrderRequest>,
) -> Result<Json<models::OrderResponse>, ApiError> {
    validate_length("menu_url", &req.menu_url)?;

    let ts = time::OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .expect("Should be able to format date");

    let order = order::ActiveModel {
        id: NotSet,
        menu_url: Set(req.menu_url),
        state: Set(1), // OPEN
        timestamp: Set(Some(ts)),
    };

    let order = order
        .insert(&state.db)
        .await
        .map_err(|e| errors::internal(e.to_string()))?;

    Ok(Json(models::order_with_entries(order, vec![])))
}

pub async fn add_entry(
    State(state): State<AppState>,
    Path(order_id): Path<i32>,
    Json(req): Json<models::AddEntryRequest>,
) -> Result<Json<models::OrderResponse>, ApiError> {
    let order = order::Entity::find_by_id(order_id)
        .one(&state.db)
        .await
        .map_err(|e| errors::internal(e.to_string()))?
        .ok_or_else(|| errors::not_found("order not found"))?;

    if order.state != 1 {
        return Err(errors::bad_request("order is not open"));
    }

    validate_length("food", &req.food)?;
    validate_length("buyer", &req.buyer)?;

    let price = napoli_lib::Millicents::from_euro_human(&req.price)
        .map_err(|_| errors::bad_request("invalid price format, expected e.g. \"13.37\""))?;

    if price.raw() > 10_000_00_000 {
        return Err(errors::bad_request("that's way too expensive"));
    }

    let entry = order_entry::ActiveModel {
        id: NotSet,
        order_id: Set(order_id),
        buyer: Set(req.buyer),
        food: Set(req.food),
        price_in_millicents: Set(price.raw()),
        paid: Set(false),
    };

    entry
        .insert(&state.db)
        .await
        .map_err(|e| errors::internal(e.to_string()))?;

    let orders = order::Entity::find_by_id(order_id)
        .find_with_related(order_entry::Entity)
        .all(&state.db)
        .await
        .map_err(|e| errors::internal(e.to_string()))?;

    let (order, entries) = orders
        .into_iter()
        .next()
        .ok_or_else(|| errors::internal("order disappeared"))?;

    Ok(Json(models::order_with_entries(order, entries)))
}

fn validate_length(name: &str, value: &str) -> Result<(), ApiError> {
    if value.len() > napoli_lib::limits::MAX_STR_LEN {
        return Err(errors::bad_request(format!(
            "{} exceeds maximum length {}",
            name,
            napoli_lib::limits::MAX_STR_LEN,
        )));
    }
    Ok(())
}
