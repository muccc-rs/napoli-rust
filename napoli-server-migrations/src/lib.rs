pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230206_005125_rename_order_state_to_state;
mod m20230206_005235_order_entry_add_price;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230206_005125_rename_order_state_to_state::Migration),
            Box::new(m20230206_005235_order_entry_add_price::Migration),
        ]
    }
}
