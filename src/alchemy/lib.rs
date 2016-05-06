#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]
pub mod distro;
pub mod su;
pub mod db;
pub mod models;
pub mod schema;
pub mod arch;

#[macro_use]
extern crate diesel;
extern crate dotenv;
