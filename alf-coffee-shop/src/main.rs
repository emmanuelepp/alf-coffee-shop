//========================================================================================
// Author:   Emmanuel Ponciano
// Created:  March 3, 2024
// Purpose:  This project demonstrates how to implement an API server in Rust using Rocket,
//           SeaORM, and JWT, exploring their integration and functionalities.
// License:  MIT
//========================================================================================

use rocket::{get, routes};

mod db;
mod migrator;
//mod entities; // Descomentado si estás listo para usar tus entidades.

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("COFFEE_SHOP_DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            db_port: std::env::var("COFFEE_SHOP_DB_PORT").unwrap_or_else(|_| "3306".to_string()),
            db_username: std::env::var("COFFEE_SHOP_DB_USERNAME").unwrap_or_else(|_| "root".to_string()),
            db_password: std::env::var("COFFEE_SHOP_DB_PASSWORD").unwrap_or_else(|_| "password".to_string()), // Cambiado a "password" para evitar usar "root" como contraseña por defecto.
            db_database: std::env::var("COFFEE_SHOP_DB_DATABASE").unwrap_or_else(|_| "coffeesshop".to_string()),
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::launch]
async fn rocket() -> _ {
    let config = AppConfig::default();
    match db::connect(&config).await {
        Ok(db) => {
            if let Err(err) = migrator::up(&db).await {
                eprintln!("Failed to run migrations: {}", err);
                std::process::exit(1);
            }
            rocket::build().mount("/", routes![index])
        },
        Err(err) => {
            eprintln!("Failed to connect to database: {}", err);
            std::process::exit(1);
        }
    }
}
