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

#[get("/")]
fn index() -> &'static str {
    "Hello, world"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
