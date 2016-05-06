extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use models::Package;
use std::env;


/// Establishes a connection to the Alchemist DB
fn establish_connection() -> PgConnection {
    //Read from the .env file where the db is located
    //at so we can connect to it.
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/// Gets all rows that match the input packages
///
/// # Examples
///
/// ```
/// let mut packages: Vec<&str> = Vec::new()
/// packages.push("sudo")
/// packages.push("postgresql")
/// let queryed = pack_query();
/// ```
///
pub fn pack_query(input_packages: Vec<&str>) -> Vec<Package> {
    //These allow us to use schema specific references
    //as well as functions like eq(), or(), and any()
    //in our querys using diesel
    use diesel::expression::dsl::*;
    use schema::packages::dsl::*;

    let connection = establish_connection();
    packages.filter(
        arch.eq(any(&input_packages))
            .or(aur.eq(any(&input_packages)))
            .or(ubuntu.eq(any(&input_packages)))
            .or(ubuntu_dev.eq(any(&input_packages)))
    )
    .load::<Package>(&connection)
    .expect("Error loading packages")
}
