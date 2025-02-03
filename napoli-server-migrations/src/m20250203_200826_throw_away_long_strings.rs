use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Truncate all text fields greater 210 length
        //                 "UPDATE \"order\" SET menu_url = SUBSTR(menu_url, 1, 210);"

        let db = manager.get_connection();

        // Truncate order.menu_url
        db.execute_unprepared("UPDATE \"order\" SET menu_url = SUBSTR(menu_url, 1, 210);")
            .await?;

        // Truncate order_entry.buyer
        db.execute_unprepared("UPDATE order_entry SET buyer = SUBSTR(buyer, 1, 210);")
            .await?;

        // Truncate order_entry.food
        db.execute_unprepared("UPDATE order_entry SET food = SUBSTR(food, 1, 210);")
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden

#[derive(Iden)]
enum Order {
    Table,
    Id,
    MenuUrl,
}

#[derive(Iden)]
enum OrderEntry {
    Table,
    Id,
    Buyer,
    Food,
}
