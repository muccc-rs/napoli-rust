use anyhow::{Context, Result};
use napoli_lib::napoli::{CreateOrderReply, CreateOrderRequest, self};
use sea_orm::{Set, TryIntoModel};

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

pub fn get_create_response_from_order(order: napoli_server_persistent_entities::order::Model) -> Result<CreateOrderReply>
{
    Ok(CreateOrderReply {
        order: Some(napoli_lib::napoli::Order {
            id: format!("{}", order.id),
            menu_url: order.menu_url,
            state: order.order_state,
            entries: vec![],
        }),
    })
}
