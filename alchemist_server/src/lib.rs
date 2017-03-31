#![feature(plugin, custom_attribute, attr_literals)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate dotenv;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod db;
pub mod schema;
pub mod models;
pub mod server_routes;
