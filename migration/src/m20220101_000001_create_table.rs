use sea_orm_migration::{prelude::*, schema::*};
use serde::{Deserialize, Serialize};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(pk_uuid(Account::Id))
                    .col(string_uniq(Account::Username))
                    .col(string_uniq(Account::Email))
                    .col(binary(Account::Password))
                    .col(double(Account::Money).default(0.0))
                    .col(big_integer(Account::Gems).default(0))
                    .col(
                        json(Account::Settings)
                            .default(serde_json::to_string(&AccountSettings::default()).unwrap()),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(City::Table)
                    .if_not_exists()
                    .col(pk_auto(City::Id))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Account::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    Username,
    Email,
    Password,
    Money,
    Gems,
    Settings,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountSettings {
    enable_notifications: bool,
}

impl Default for AccountSettings {
    fn default() -> Self {
        Self {
            enable_notifications: true,
        }
    }
}

#[derive(DeriveIden)]
enum City {
    Table,
    Id,
    Name,
    Owner,
    Region,
}

// #[derive(Debug, DeriveColumn)]
// struct Region {
//     start: Coordinates,
//     end: Coordinates,
// }

// #[derive(Debug, DeriveValueType, Serialize, Deserialize)]
// struct Coordinates((u64, u64));

// impl TryGetable for Coordinates {
//     fn try_get_by<I: sea_orm::ColIdx>(res: &sea_orm::QueryResult, index: I) -> Result<Self, sea_orm::TryGetError> {

//     }
// }
