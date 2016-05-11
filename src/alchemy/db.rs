extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use models::Package;
use std::env;
use std::collections::HashSet;


/// Establishes a connection to the Alchemist DB
fn establish_connection() -> SqliteConnection {
    //Read from the .env file where the db is located
    //at so we can connect to it.
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
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
pub fn pack_query(input_packages: HashSet<String>) -> HashSet<Package> {
    use schema::packages::dsl::*;

    let connection = establish_connection();

    let mut output: HashSet<Package> = HashSet::new();

    for i in input_packages {
        //While this might look like O(n^2) complexity
        //it's more closer to O(n) since most querys only
        //return one or 2 results really. Still this seemed
        //to be the only way to implement it well
        let results = packages.filter(
            arch.eq(&i)
            .or(aur.eq(&i))
            .or(ubuntu.eq(&i))
            .or(ubuntu_dev.eq(&i))
        )
        .get_results::<Package>(&connection)
        .unwrap_or(vec![Package::empty()]);
        for j in results {
            output.insert(j);
        }
    }

    output
}
