//========================================================================================
// Author:   Emmanuel Ponciano
// Created:  March 3, 2024
// Purpose:  This project demonstrates how to implement an API server in Rust using Rocket,
//           SeaORM, and JWT, exploring their integration and functionalities.
// License:  MIT
//========================================================================================

#[macro_use]
extern crate rocket;

mod Migrator;

pub struct AppConfig {
    db_type: String,
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: STring,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var(key: "COFFEESSHOP_DB_HOST").unwrap_or(default: "localhost".to_string()),
            db_host: std::env::var(key: "COFFEESSHOP_DB_PORT").unwrap_or(default: "3306".to_string()),
            db_host: std::env::var(key: "COFFEESSHOP_DB_USERNAME").unwrap_or(default: "root".to_string()),
            db_host: std::env::var(key: "COFFEESSHOP_DB_PASSWORD").unwrap_or(default: "".to_string()),
            db_host: std::env::var(key: "COFFEESSHOP_DB_DATABASE").unwrap_or(default: "coffeesshop".to_string()),
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
