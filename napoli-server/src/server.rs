use tonic::{transport::Server, Response, Status};
use napoli_lib::napoli::{GetOrdersRequest, GetOrdersReply};
use napoli_lib::napoli::order_service_server::{OrderService, OrderServiceServer};
use napoli_lib::create_example_order;

#[derive(Default)]
pub struct MyOrderServiceServer {}

#[tonic::async_trait]
impl OrderService for MyOrderServiceServer {
    async fn get_orders(&self, request: tonic::Request<GetOrdersRequest>) -> Result<Response<GetOrdersReply>, Status> {
        println!("Got a request: {:?}", request);
        let reply = GetOrdersReply {
            orders: vec![create_example_order()],
        };
        Ok(Response::new(reply))
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyOrderServiceServer::default();

    println!("MyOrderServiceServer listening on {}", addr);

    Server::builder()
        .add_service(OrderServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}