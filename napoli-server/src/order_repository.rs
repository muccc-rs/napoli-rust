use anyhow::Result;

type IdType = i32;

trait OrderRepositoryBackend {
    fn get_orders(&self) -> Result<Vec<Order>>;
    fn get_open_orders(&self) -> Result<Vec<Order>>;
    fn get_order(&self, id: IdType) -> Result<Order>;

    fn create_order(&self, menu_url: String) -> Result<Order>;
}