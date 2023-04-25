use tonic_web_wasm_client::Client;
use yew::prelude::*;

use napoli_lib::napoli::order_service_client as npb_grpc;
use napoli_lib::napoli::{self as npb, ObjectId};

#[derive(Debug, Clone)]
pub struct ServiceError(String);

impl ServiceError {
    pub fn html(&self) -> Html {
        html! {
            { self.0.clone() }
        }
    }
}

impl From<tonic::Status> for ServiceError {
    fn from(other: tonic::Status) -> Self {
        ServiceError(other.message().into())
    }
}

type Result<T> = std::result::Result<T, ServiceError>;

pub struct Napoli {
    pub client: npb_grpc::OrderServiceClient<Client>,
}

impl Napoli {
    pub fn new(backend_url: String) -> Self {
        Napoli {
            client: npb_grpc::OrderServiceClient::new(Client::new(backend_url)),
        }
    }

    pub async fn get_orders(&mut self) -> Result<Vec<npb::Order>> {
        let orders = self.client.get_orders(npb::GetOrdersRequest {}).await?;
        Ok(orders.into_inner().orders)
    }

    pub async fn set_order_entry_paid(
        &mut self,
        order_id: ObjectId,
        order_entry_id: ObjectId,
        paid: bool,
    ) -> Result<npb::Order> {
        let order = self
            .client
            .set_order_entry_paid(npb::SetOrderEntryPaidRequest {
                order_id,
                order_entry_id,
                paid,
            })
            .await?;
        Ok(order.into_inner().order.expect("fucked up"))
    }

    pub async fn create_order(&mut self, menu_url: String) -> Result<npb::Order> {
        let order = self
            .client
            .create_order(npb::CreateOrderRequest { menu_url: menu_url })
            .await?;
        Ok(order.into_inner().order.expect("fucked up"))
    }

    pub async fn add_order_entry(
        &mut self,
        request: npb::AddOrderEntryRequest,
    ) -> Result<npb::Order> {
        let order = self.client.add_order_entry(request).await?;
        Ok(order.into_inner().order.expect("fucked up"))
    }

    pub async fn remove_order_entry(
        &mut self,
        request: npb::OrderEntryRequest,
    ) -> Result<npb::Order> {
        let order = self.client.remove_order_entry(request).await?;
        Ok(order.into_inner().order.expect("fucked up"))
    }
}
