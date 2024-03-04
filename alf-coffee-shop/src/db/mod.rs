use crate::AppConfig; 

use sea_orm::{Database, DatabaseConnection, DbErr, ConnectOptions}; 

pub async fn connect(config: &AppConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new(
        format!("mysql://{}:{}@{}:{}/{}", config.db_username, config.db_password, config.db_host, config.db_port, config.db_database)
    );

    opts.sqlx_logging(false);

    Database::connect(opts).await
}
