use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Order {
    Table,
    Id,
    MenuUrl,
    #[allow(clippy::enum_variant_names)]
    OrderState,
}

#[derive(Iden)]
enum OrderEntry {
    Table,
    Id,
    OrderId,
    Buyer,
    Food,
    Paid,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .col(
                        ColumnDef::new(Order::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Order::MenuUrl).text().not_null())
                    .col(
                        ColumnDef::new(Order::OrderState)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrderEntry::Table)
                    .col(
                        ColumnDef::new(OrderEntry::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OrderEntry::OrderId).unsigned().not_null())
                    .col(ColumnDef::new(OrderEntry::Buyer).text().not_null())
                    .col(ColumnDef::new(OrderEntry::Food).text().not_null())
                    .col(
                        ColumnDef::new(OrderEntry::Paid)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderEntry::Table, OrderEntry::OrderId)
                            .to(Order::Table, Order::Id)
                            .name("fk_order_entry_order_id"),
                    )
                    .to_owned(),
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
