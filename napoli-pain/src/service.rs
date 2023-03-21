#[derive(Debug)]
pub struct ServiceError(String);

type Result<T> = std::result::Result<T, ServiceError>;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Order {
    pub id: i32,
    pub menu_url: &'static str,
}

pub struct Napoli {
}

impl Napoli {
    pub fn get_orders(&self) -> Result<Vec<Order>> {
        return Ok(vec![
            Order {
                id: 2137,
                menu_url: "https://example.com",
            },
            Order {
                id: 2137,
                menu_url: "https://example.com",
            },
            Order {
                id: 2137,
                menu_url: "https://example.com",
            },
            Order {
                id: 2137,
                menu_url: "https://example.com",
            },
            Order {
                id: 2137,
                menu_url: "https://example.com",
            },
            Order {
                id: 2137,
                menu_url: "https://example.com",
            },
            Order {
                id: 2137,
                menu_url: "https://example.com",
            },
            Order {
                id: 2137,
                menu_url: "https://example.com",
            },
            Order {
                id: 2137,
                menu_url: "https://example.com",
            },
            Order {
                id: 2137,
                menu_url: "https://example.com",
            },
            Order {
                id: 2137,
                menu_url: "https://example.com",
            },
        ]);
    }
}