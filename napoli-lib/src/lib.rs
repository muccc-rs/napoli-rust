pub mod millicents;
pub use millicents::Millicents;

pub mod napoli {
    tonic::include_proto!("napoli");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("napoli_descriptor");

    pub type ObjectId = i32;

    impl Eq for Order {}
    impl Eq for OrderEntry {}
}

pub fn create_example_order() -> napoli::Order {
    let mut order = napoli::Order {
        id: 1,
        menu_url: "https://www.napoli-pizza.com/menu".to_string(),
        state: napoli::OrderState::Open.into(),
        ..Default::default()
    };

    let entry = napoli::OrderEntry {
        id: 1,
        buyer: "John".to_string(),
        food: "pizza".to_string(),
        price_deprecated: 0.0,
        price_in_millicents: 10000,
        paid: false,
    };
    order.entries.push(entry);

    let entry = napoli::OrderEntry {
        id: 2,
        buyer: "Jane".to_string(),
        food: "pizza".to_string(),
        price_deprecated: 0.0,
        price_in_millicents: 10000,
        paid: false,
    };

    order.entries.push(entry);

    order
}
