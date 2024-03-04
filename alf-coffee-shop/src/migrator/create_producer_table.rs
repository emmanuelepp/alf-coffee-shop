use sea_orm_migration::prelude::*;
use super::create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct CreateProducerTable;

#[async_trait::async_trait]
impl MigrationTrait for CreateProducerTable {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Producer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Producer::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Producer::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-producer-user-id")
                            .from(Producer::Table, Producer::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Producer::CompanyName).string().not_null())
                    .col(ColumnDef::new(Producer::Bio).string().not_null())
                    .col(
                        ColumnDef::new(Producer::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(Producer::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Producer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Producer {
    Table,
    Id,
    UserId,
    CompanyName,
    Bio,
    CreatedAt,
    UpdatedAt,
}
