use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
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
                    .col(ColumnDef::new(Coffees::Name).string().not_null())
                    .col(ColumnDef::new(Coffees::Type).string().not_null())
                    .col(ColumnDef::new(Coffees::Size).string().not_null())
                    .col(
                        ColumnDef::new(Coffees::CreatedAt)
                        .timestamp()
                        .extra("DEFAULT CURRENT_TIMESTAMP"
                        .to_owned())
                    )
                    .col(
                        ColumnDef::new(Coffees::UpdatedAt)
                        .timestamp()
                        .extra("DEFAULT CURRENT_TIMESTAMP"
                        .to_owned()))
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
enum Coffees {
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
