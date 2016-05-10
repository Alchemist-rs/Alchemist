extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use models::Package;
use std::env;
use std::collections::HashSet;


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
/// let mut packages: HashSet<&str> = HashSet::new()
/// packages.insert("sudo")
/// packages.insert("postgresql")
/// let queryed = pack_query(packages);
/// ```
///
pub fn pack_query(mut input_packages: HashSet<String>) -> HashSet<Package> {
    //These allow us to use schema specific references
    //as well as functions like eq(), or(), and any()
    //in our querys using diesel
    use diesel::expression::dsl::*;
    use schema::packages::dsl::*;

    let connection = establish_connection();

    //TODO Establish a way for Diesel to accept a HashSet
    let query_input = input_packages
        .drain().collect::<Vec<String>>();

    let query_output = packages.filter(
        arch.eq(any(&query_input))
            .or(aur.eq(any(&query_input)))
            .or(ubuntu.eq(any(&query_input)))
            .or(ubuntu_dev.eq(any(&query_input)))
        )
        .load::<Package>(&connection)
        .expect("Error loading packages");

    let mut output: HashSet<Package> = HashSet::new();
    for i in query_output {
        output.insert(i);
    }

    output
}
