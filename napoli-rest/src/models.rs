use napoli_server_persistent_entities::{order, order_entry};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OrderResponse {
    pub id: i32,
    pub menu_url: String,
    pub state: String,
    pub timestamp: String,
    pub entries: Vec<EntryResponse>,
}

#[derive(Serialize)]
pub struct OrderSummaryResponse {
    pub id: i32,
    pub menu_url: String,
    pub state: String,
    pub timestamp: String,
}

#[derive(Serialize)]
pub struct EntryResponse {
    pub id: i32,
    pub buyer: String,
    pub food: String,
    pub price_in_millicents: i64,
    pub paid: bool,
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub menu_url: String,
}

#[derive(Deserialize)]
pub struct AddEntryRequest {
    pub buyer: String,
    pub food: String,
    pub price: String,
}

#[derive(Deserialize)]
pub struct OrdersQuery {
    pub summary: Option<bool>,
}

fn state_to_string(state: i32) -> String {
    match state {
        1 => "OPEN".into(),
        2 => "CLOSED".into(),
        3 => "DONE".into(),
        _ => "INVALID".into(),
    }
}

pub fn order_with_entries(
    order: order::Model,
    mut entries: Vec<order_entry::Model>,
) -> OrderResponse {
    entries.sort_by_key(|e| e.id);
    OrderResponse {
        id: order.id,
        menu_url: order.menu_url,
        state: state_to_string(order.state),
        timestamp: order.timestamp.unwrap_or_default(),
        entries: entries.into_iter().map(entry_response).collect(),
    }
}

pub fn order_summary(order: order::Model) -> OrderSummaryResponse {
    OrderSummaryResponse {
        id: order.id,
        menu_url: order.menu_url,
        state: state_to_string(order.state),
        timestamp: order.timestamp.unwrap_or_default(),
    }
}

fn entry_response(entry: order_entry::Model) -> EntryResponse {
    EntryResponse {
        id: entry.id,
        buyer: entry.buyer,
        food: entry.food,
        price_in_millicents: entry.price_in_millicents,
        paid: entry.paid,
    }
}
