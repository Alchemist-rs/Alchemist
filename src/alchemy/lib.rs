#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]
///Top Level Module File
///All of the various available public modules that can be used are listed here

///All common functions related to distributions as well as distribution specific
///functions are available in the distro module
pub mod distro;
pub mod su;
pub mod db;
pub mod models;
pub mod schema;
pub mod arch;
pub mod void;
pub mod ubuntu;
pub mod debian;

#[macro_use]
extern crate diesel;
extern crate dotenv;
