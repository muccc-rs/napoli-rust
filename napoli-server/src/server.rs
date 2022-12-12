use tonic::{transport::Server, Response, Status};
use napoli_lib::napoli::{GetOrdersRequest, GetOrdersReply, FILE_DESCRIPTOR_SET};
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


    let reflection = tonic_reflection::server::Builder::configure().register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET).build().unwrap();

    Server::builder()
        .add_service(OrderServiceServer::new(greeter))
        .add_service(reflection)
        .serve(addr)
        .await?;

    Ok(())
}