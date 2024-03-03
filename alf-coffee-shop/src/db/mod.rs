use sea_orm::{Database, DatabaseConnection, DbErr};

use crate::AppConfig;

pub async fn connect(config: &AppConfig) -> Result<DatabaseConnection, DbErr> {
    let connection_string = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.db_username, config.db_password, config.db_host, config.db_port, config.db_database,
    );

    match Database::connect(&connection_string).await {
        Ok(connection) => Ok(connection),
        Err(e) => {
            eprintln!("Error connecting to database: {:?}", e);
            Err(e)
        }
    }
}
