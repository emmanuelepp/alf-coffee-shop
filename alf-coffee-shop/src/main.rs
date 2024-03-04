use rocket::{Build, Rocket, fairing::AdHoc};
use rocket::futures::future::BoxFuture;

#[macro_use] extern crate rocket;

mod db;
mod migrator;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
}

impl AppConfig {
    fn from_env() -> Self {
        Self {
            db_host: std::env::var("COFFEE_SHOP_DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            db_port: std::env::var("COFFEE_SHOP_DB_PORT").unwrap_or_else(|_| "3306".to_string()),
            db_username: std::env::var("COFFEE_SHOP_DB_USERNAME").unwrap_or_else(|_| "root".to_string()),
            db_password: std::env::var("COFFEE_SHOP_DB_PASSWORD").unwrap_or_else(|_| "root".to_string()),
            db_database: std::env::var("COFFEE_SHOP_DB_DATABASE").unwrap_or_else(|_| "coffeeshop".to_string()),
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn run_migrations(rocket: Rocket<Build>) -> BoxFuture<'static, Rocket<Build>> {
    Box::pin(async move {
        let config = AppConfig::from_env();
        match db::connect(&config).await {
            Ok(db) => {
                if migrator::up(&db).await.is_ok() {
                    rocket
                } else {
                    eprintln!("Failed to run database migrations.");
                    rocket
                }
            },
            Err(e) => {
                eprintln!("Failed to connect to database: {:?}", e);
                rocket
            }
        }
    })
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .attach(AdHoc::on_ignite("Database Migrations", run_migrations))
}
