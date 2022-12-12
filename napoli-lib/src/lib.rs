pub mod napoli {
    tonic::include_proto!("napoli");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("napoli_descriptor");
}

pub fn create_example_order() -> napoli::Order {
    let mut order = napoli::Order::default();
    order.id = "order-1".to_string();
    order.menu_url = "https://www.napoli-pizza.com/menu".to_string();
    order.is_open = true;

    let mut entry = napoli::OrderEntry::default();
    entry.id = "entry-1".to_string();
    entry.food = "pizza".to_string();
    entry.buyer = "John".to_string();
    entry.quantity = 1;
    order.entries.push(entry);

    let mut entry = napoli::OrderEntry::default();
    entry.id = "entry-2".to_string();
    entry.food = "pizza".to_string();
    entry.buyer = "Jane".to_string();
    entry.quantity = 1;
    order.entries.push(entry);

    order
}