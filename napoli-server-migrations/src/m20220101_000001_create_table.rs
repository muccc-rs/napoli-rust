use sea_orm_migration::{prelude::*, sea_orm::Schema};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        // Setup Schema helper
        let schema = Schema::new(manager.get_database_backend());

        // Derive from Entity
        // Execute create table statement
        manager
            .create_table(
                schema.create_table_from_entity(napoli_server_persistent_entities::order::Entity),
            )
            .await?;

        manager
            .create_table(
                schema.create_table_from_entity(
                    napoli_server_persistent_entities::order_entry::Entity,
                ),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(OrderEntry::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Order {
    Table,
}

#[derive(Iden)]
enum OrderEntry {
    Table,
}
