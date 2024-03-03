use sea_orm::DatabaseConnection;
use sea_orm_migration::{SchemaManager, prelude::*};

mod create_user_table;
mod create_producer_table;
mod create_coffees_table;

pub use create_user_table::*;
pub use create_producer_table::*;
pub use create_coffees_table::*;

pub async fn up(db: &DatabaseConnection) -> Result<(), DbErr> {
    let manager = SchemaManager::new(db);
    let migrations = Migrator::migrations();
    for migration in migrations {
        if let Err(e) = migration.up(&manager).await {
            eprintln!("Error running migration: {:?}", e);
            return Err(e);
        }
    }
    Ok(())
}

pub struct Migrator;

impl Migrator {
    pub fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(CreateUserTable),
            Box::new(CreateProducerTable),
            Box::new(CreateCoffeesTable),
        ]
    }
}
