use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub menu_url: String,
    pub order_state: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    OrderEntry,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::OrderEntry => Entity::has_many(super::order_entry::Entity).into(),
        }
    }
}

impl Related<super::order_entry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrderEntry.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
