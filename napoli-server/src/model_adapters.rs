use napoli_lib::{
    napoli::{AddOrderEntryRequest, CreateOrderRequest, SingleOrderReply},
    Millicents,
};
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
    // This is to support the migration from price to price_in_millicents for the protocol
    let price_in_millicents = match if request.price_deprecated > 0.0 {
        napoli_lib::Millicents::from_euro_float(request.price_deprecated)
    } else {
        napoli_lib::Millicents::from_raw(request.price_in_millicents)
    } {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to parse: {:?}", e);
            return None;
        }
    };

    Some(
        napoli_server_persistent_entities::order_entry::ActiveModel {
            id: NotSet,
            order_id: Set(request.order_id),
            buyer: Set(request.buyer),
            food: Set(request.food),
            price_in_millicents: Set(price_in_millicents.raw()),
            paid: Set(false),
        },
    )
}

pub fn database_order_to_tonic_order(
    order: napoli_server_persistent_entities::order::Model,
    order_entries: impl Iterator<Item = napoli_server_persistent_entities::order_entry::Model>,
) -> napoli_lib::napoli::Order {
    let mut order_entries: Vec<_> = order_entries.collect();
    order_entries.sort_by_key(|entry| entry.id);
    let order_entries = order_entries.into_iter();

    napoli_lib::napoli::Order {
        id: order.id,
        menu_url: order.menu_url,
        state: order.state,
        entries: order_entries
            .map(|entry| {
                // TODO Add tainted flag to the protocol
                let price = match napoli_lib::Millicents::from_raw(entry.price_in_millicents) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Failed to parse: {:?}", e);
                        Millicents::zero()
                    }
                };

                napoli_lib::napoli::OrderEntry {
                    id: entry.id,
                    buyer: entry.buyer.to_owned(),
                    food: entry.food.to_owned(),
                    price_deprecated: price.to_euro_float(),
                    price_in_millicents: entry.price_in_millicents,
                    paid: entry.paid,
                }
            })
            .collect(),
    }
}

pub fn make_single_order_reply(
    order: napoli_server_persistent_entities::order::Model,
    mut order_entries: Vec<napoli_server_persistent_entities::order_entry::Model>,
) -> SingleOrderReply {
    order_entries.sort_by_key(|entry| entry.id);

    SingleOrderReply {
        order: Some(database_order_to_tonic_order(
            order,
            order_entries.into_iter(),
        )),
    }
}
