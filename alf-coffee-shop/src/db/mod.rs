// En db.rs

use crate::AppConfig;
use sea_orm::{Database, DatabaseConnection, DbErr, ConnectOptions};

pub async fn connect(config: &AppConfig) -> Result<DatabaseConnection, DbErr> {

    let port: u16 = config.db_port.parse().unwrap_or_else(|_| 3306); 

    let mut opts = ConnectOptions::new(format!(
        "mysql://{}:{}@{}:{}/{}",
        config.db_username, config.db_password, config.db_host, port, config.db_database
    ));

    opts.sqlx_logging(false);

    Database::connect(opts).await
}
