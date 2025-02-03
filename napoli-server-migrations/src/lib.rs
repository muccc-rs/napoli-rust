pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230206_005125_rename_order_state_to_state;
mod m20230206_005235_order_entry_add_price;
mod m20230425_2051_price;
mod m20241126_202903_add_date_to_order;
mod m20250203_200826_throw_away_long_strings;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230206_005125_rename_order_state_to_state::Migration),
            Box::new(m20230206_005235_order_entry_add_price::Migration),
            Box::new(m20230425_2051_price::Migration),
            Box::new(m20241126_202903_add_date_to_order::Migration),
            Box::new(m20250203_200826_throw_away_long_strings::Migration),
        ]
    }
}
