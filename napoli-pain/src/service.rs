use tonic_web_wasm_client::Client;
use yew::prelude::*;

use napoli_lib::napoli as npb;
use napoli_lib::napoli::order_service_client as npb_grpc;

#[derive(Debug)]
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
    pub backend_url: String,
}

impl Napoli {
    pub async fn get_orders(&self) -> Result<Vec<npb::Order>> {
        let mut ocl = npb_grpc::OrderServiceClient::new(Client::new(self.backend_url.clone()));
        let orders = ocl.get_orders(npb::GetOrdersRequest {}).await?;
        Ok(orders.into_inner().orders)
    }

    pub async fn set_order_entry_paid(
        &self,
        order_id: u32,
        order_entry_id: u32,
        paid: bool,
    ) -> Result<npb::Order> {
        let mut ocl = npb_grpc::OrderServiceClient::new(Client::new(self.backend_url.clone()));
        let order = ocl
            .set_order_entry_paid(npb::SetOrderEntryPaidRequest {
                order_id,
                order_entry_id,
                paid,
            })
            .await?;
        Ok(order.into_inner().order.expect("fucked up"))
    }
}
