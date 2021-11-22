use chrono::{FixedOffset, TimeZone, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: i32,
    pub first_name: String,
    created: i64,
}

impl Model {
    pub fn created(&self) -> String {
        let date = Utc
            .timestamp_millis(self.created)
            //东八区
            .with_timezone(&FixedOffset::east(8 * 3600));
        date.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        todo!()
    }
}

impl ActiveModelBehavior for ActiveModel {}
