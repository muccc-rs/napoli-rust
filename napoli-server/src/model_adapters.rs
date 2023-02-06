use napoli_lib::napoli::{AddOrderEntryRequest, CreateOrderRequest, SingleOrderReply};
use sea_orm::{ActiveValue::NotSet, Set};

pub fn get_order_from_create_request(
    request: CreateOrderRequest,
) -> Option<napoli_server_persistent_entities::order::ActiveModel> {
    Some(napoli_server_persistent_entities::order::ActiveModel {
        id: NotSet,
        menu_url: Set(request.menu_url),
        // You can replace with: #[sea_orm(default_value="1")] in the model definition,
        // but loose the ability to use the enum directly there, this is why we do it here
        state: Set(napoli_lib::napoli::OrderState::Open as i32),
    })
}

pub fn get_order_entry_from_add_request(
    request: AddOrderEntryRequest,
) -> Option<napoli_server_persistent_entities::order_entry::ActiveModel> {
    Some(
        napoli_server_persistent_entities::order_entry::ActiveModel {
            id: NotSet,
            order_id: Set(request.order_id),
            buyer: Set(request.buyer),
            food: Set(request.food),
            price: Set(request.price),
            paid: Set(false),
        },
    )
}

pub fn database_order_to_tonic_order(
    order: napoli_server_persistent_entities::order::Model,
    order_entries: impl Iterator<Item = napoli_server_persistent_entities::order_entry::Model>,
) -> napoli_lib::napoli::Order {
    napoli_lib::napoli::Order {
        id: order.id,
        menu_url: order.menu_url,
        state: order.state,
        entries: order_entries
            .map(|entry| napoli_lib::napoli::OrderEntry {
                id: entry.id,
                buyer: entry.buyer.to_owned(),
                food: entry.food.to_owned(),
                price: entry.price,
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
        order: Some(database_order_to_tonic_order(
            order,
            order_entries.into_iter(),
        )),
    }
}
