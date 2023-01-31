use anyhow::Result;
use napoli_lib::napoli::{CreateOrderRequest, SingleOrderReply};
use sea_orm::{ModelTrait, Set};

pub fn get_order_from_create_request(
    request: CreateOrderRequest,
) -> Option<napoli_server_persistent_entities::order::ActiveModel> {
    Some(napoli_server_persistent_entities::order::ActiveModel {
        menu_url: Set(request.menu_url),
        // You can replace with: #[sea_orm(default_value="1")] in the model definition,
        // but loose the ability to use the enum directly there, this is why we do it here
        order_state: Set(napoli_lib::napoli::OrderState::Open as i32),
        ..Default::default() // all other attributes are `NotSet`
    })
}

pub fn max_told_me_so(
    order: napoli_server_persistent_entities::order::Model,
    order_entries: impl Iterator<Item = napoli_server_persistent_entities::order_entry::Model>,
) -> napoli_lib::napoli::Order {
    napoli_lib::napoli::Order {
        id: order.id,
        menu_url: order.menu_url,
        state: order.order_state,
        entries: order_entries
            .map(|entry| napoli_lib::napoli::OrderEntry {
                id: entry.id,
                buyer: entry.buyer.to_owned(),
                food: entry.food.to_owned(),
                paid: entry.paid,
            })
            .collect(),
    }
}

pub fn make_single_order_reply(
    order: napoli_server_persistent_entities::order::Model,
    order_entries: Vec<napoli_server_persistent_entities::order_entry::Model>,
) -> SingleOrderReply {
    SingleOrderReply {
        order: Some(max_told_me_so(order, order_entries.into_iter())),
    }
}
