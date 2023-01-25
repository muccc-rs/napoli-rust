pub mod napoli {
    tonic::include_proto!("napoli");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("napoli_descriptor");
}

pub fn create_example_order() -> napoli::Order {
    let mut order = napoli::Order::default();
    order.id = 1;
    order.menu_url = "https://www.napoli-pizza.com/menu".to_string();
    order.state = napoli::OrderState::Open.into();

    let mut entry = napoli::OrderEntry::default();
    entry.id = 1;
    entry.food = "pizza".to_string();
    entry.buyer = "John".to_string();
    order.entries.push(entry);

    let mut entry = napoli::OrderEntry::default();
    entry.id = 2;
    entry.food = "pizza".to_string();
    entry.buyer = "Jane".to_string();
    order.entries.push(entry);

    order
}
