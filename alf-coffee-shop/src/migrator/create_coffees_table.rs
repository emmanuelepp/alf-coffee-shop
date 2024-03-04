use sea_orm_migration::prelude::*;
use super::create_user_table::User;
use super::create_producer_table::Producer;

#[derive(DeriveMigrationName)]
pub struct CreateCoffeesTable;

#[async_trait::async_trait]
impl MigrationTrait for CreateCoffeesTable {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Coffees::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Coffees::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Coffees::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-coffees-user-id")
                            .from(Coffees::Table, Coffees::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Coffees::ProducerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-coffees-producer-id")
                            .from(Coffees::Table, Coffees::ProducerId)
                            .to(Producer::Table, Producer::Id),
                    )
                    .col(ColumnDef::new(Coffees::Name).string().not_null())
                    .col(ColumnDef::new(Coffees::Type).string().not_null())
                    .col(ColumnDef::new(Coffees::Size).string().not_null())
                    .col(
                        ColumnDef::new(Coffees::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(Coffees::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Coffees::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Coffees {
    Table,
    Id,
    UserId,
    ProducerId,
    Name,
    Type,
    Size,
    CreatedAt,
    UpdatedAt,
}
