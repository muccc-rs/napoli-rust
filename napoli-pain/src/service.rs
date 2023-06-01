use tonic_web_wasm_client::Client;
use yew::prelude::*;

use napoli_lib::napoli as npb;

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

impl From<&str> for ServiceError {
    fn from(other: &str) -> Self {
        ServiceError(other.into())
    }
}

type Result<T> = std::result::Result<T, ServiceError>;

pub struct Napoli {
    pub client: npb::order_service_client::OrderServiceClient<Client>,
}

impl Napoli {
    pub fn new(backend_url: String) -> Self {
        Napoli {
            client: npb::order_service_client::OrderServiceClient::new(Client::new(backend_url)),
        }
    }

    pub async fn get_orders(&mut self) -> Result<Vec<npb::Order>> {
        let orders = self.client.get_orders(npb::GetOrdersRequest {}).await?;
        Ok(orders.into_inner().orders)
    }

    pub async fn set_order_entry_paid(
        &mut self,
        order_id: npb::ObjectId,
        order_entry_id: npb::ObjectId,
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
            .create_order(npb::CreateOrderRequest { menu_url })
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

    pub async fn stream_order_updates(
        &mut self,
        order_id: i32,
    ) -> Result<tonic::Streaming<npb::SingleOrderReply>> {
        let request = npb::GetOrderRequest { order_id };
        let res = self.client.stream_order_updates(request).await?;
        Ok(res.into_inner())
    }
}
