use bcrypt::{hash, DEFAULT_COST};
use entity::{account, prelude::*};
use sea_orm::{ActiveModelTrait, DbConn, DbErr, DeleteResult, EntityTrait, Set};
use uuid::Uuid;

use crate::forms::{CreateAccount, DeleteAccount};

pub struct Mutation;

impl Mutation {
    pub async fn create_account(
        conn: &DbConn,
        form: CreateAccount,
    ) -> Result<account::Model, DbErr> {
        account::ActiveModel {
            id: Set(Uuid::now_v7()),
            username: Set(form.username.clone()),
            email: Set(form.email.clone()),
            password: Set(hash(form.password.clone(), DEFAULT_COST).unwrap().into()),
            ..Default::default()
        }
        .insert(conn)
        .await
    }

    pub async fn delete_account(conn: &DbConn, form: DeleteAccount) -> Result<DeleteResult, DbErr> {
        let acc: account::ActiveModel = Account::find_by_id(form.id)
            .one(conn)
            .await?
            .ok_or(DbErr::Custom("Cannot find account with this id.".into()))
            .map(Into::into)?;
        acc.delete(conn).await
    }
}
