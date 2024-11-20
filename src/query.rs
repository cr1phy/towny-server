use entity::{account, prelude::*};
use sea_orm::{DbConn, DbErr, EntityTrait};
use uuid::Uuid;

pub struct Query;

impl Query {
    pub async fn find_account_by_id(
        db: &DbConn,
        id: Uuid,
    ) -> Result<Option<account::Model>, DbErr> {
        Account::find_by_id(id).one(db).await
    }
}
