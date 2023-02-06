use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/* Migration Purpose:
 * In table order_entry, add column price
 */

#[derive(Iden)]
enum OrderEntry {
    Table,
    Price,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(OrderEntry::Table)
                    .add_column(ColumnDef::new(OrderEntry::Price).double().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(OrderEntry::Table)
                    .drop_column(OrderEntry::Price)
                    .to_owned(),
            )
            .await
    }
}
