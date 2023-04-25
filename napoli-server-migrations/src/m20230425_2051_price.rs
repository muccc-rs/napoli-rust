use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/* Migration Purpose:
 * Change price to be in millicents and an integer
 */

#[derive(Iden)]
enum OrderEntry {
    Table,
    Price,
    PriceInMillicents,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add new column price_in_millicents
        manager
            .alter_table(
                Table::alter()
                    .table(OrderEntry::Table)
                    .add_column(
                        ColumnDef::new(OrderEntry::PriceInMillicents)
                            .big_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await
            .expect("Failed to add price_in_millicents column.");

        // Fill priceinmillicents from price column
        manager
            .exec_stmt(
                Query::update()
                    .table(OrderEntry::Table)
                    .values([(
                        OrderEntry::PriceInMillicents,
                        Expr::col(OrderEntry::Price)
                            .mul(100000)
                            .cast_as(Alias::new("integer")),
                    )])
                    .to_owned(),
            )
            .await
            .expect("Failed to fill price_in_millicents from price column.");

        // Drop price column
        manager
            .alter_table(
                Table::alter()
                    .table(OrderEntry::Table)
                    .drop_column(OrderEntry::Price)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add price column
        manager
            .alter_table(
                Table::alter()
                    .table(OrderEntry::Table)
                    .add_column(
                        ColumnDef::new(OrderEntry::Price)
                            .big_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await
            .expect("Failed to add price column.");

        // Fill price column from priceinmillicents column
        manager
            .exec_stmt(
                Query::update()
                    .table(OrderEntry::Table)
                    .values([(
                        OrderEntry::Price,
                        Expr::col(OrderEntry::PriceInMillicents).div(100000),
                    )])
                    .to_owned(),
            )
            .await
            .expect("Failed to fill price column from price_in_millicents column.");

        // Drop priceinmillicents column
        manager
            .alter_table(
                Table::alter()
                    .table(OrderEntry::Table)
                    .drop_column(OrderEntry::PriceInMillicents)
                    .to_owned(),
            )
            .await
    }
}
