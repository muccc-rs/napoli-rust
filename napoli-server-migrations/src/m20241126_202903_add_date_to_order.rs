use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/**
 * Add
 */

#[derive(Iden)]
enum Order {
    Table,
    Timestamp,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Order::Table)
                    .add_column(ColumnDef::new(Order::Timestamp).text())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Order::Table)
                    .drop_column(Order::Timestamp)
                    .to_owned(),
            )
            .await
    }
}
