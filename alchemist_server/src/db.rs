extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use models::{Package, Distro};
use std::env;
use std::string::String;

/// Establishes a connection to the Alchemist DB
fn establish_connection() -> PgConnection {
    // Read from the .env file where the db is located
    // at so we can connect to it.
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

/// Gets all rows that match the input packages
///
/// # Examples
///
/// ```rust
/// let mut packages: Vec<String> = Vec::new();
/// packages.push("sudo".to_string());
/// packages.push("postgresql".to_string());
/// let queryed = alchemist_server::db::pkg_query(packages);
/// ```
///
pub fn pkg_query(input_packages : Vec<String>) -> Vec<Package> {
    use schema::packages::dsl::*;

    let connection = establish_connection();

    let mut output : Vec<Package> = Vec::new();

    for i in input_packages {
        // While this might look like O(n^2) complexity
        // it's more closer to O(n) since most querys only
        // return one or 2 results really. Still this seemed
        // to be the only way to implement it well
        let results = packages
            .filter(arch.eq(&i)
                        .or(ubuntu.eq(&i))
                        .or(mint.eq(&i))
                        .or(debain.eq(&i))
                        .or(gentoo.eq(&i))
                        .or(void.eq(&i))
                        .or(mac.eq(&i))
                        .or(void.eq(&i))
                        .or(freebsd.eq(&i))
                        .or(netbsd.eq(&i))
                        .or(openbsd.eq(&i)))
            .get_results::<Package>(&connection)
            .unwrap_or(vec![Package::empty()]);
        for j in results {
            output.push(j);
        }
    }

    output
}

/// Convert package names from other distros to the one being run by the user currently
pub fn convert_to_distro(input_packages : Vec<String>, distro : &Distro) -> Vec<String> {
    let results = pkg_query(input_packages);

    // All querys will either be a string or '' in the db allowing us to
    // use is_empty()
    // Finds out what distro is used and inserts the proper conversions
    // into the HashSet to be returned from the function
    match distro {
        &Distro::Ubuntu => {
            results
                .into_iter()
                .filter(|package| !package.ubuntu.is_empty())
                .map(|package| package.ubuntu)
                .collect::<Vec<String>>()
        },
        &Distro::Void => {
            results
                .into_iter()
                .filter(|package| !package.void.is_empty())
                .map(|package| package.void)
                .collect::<Vec<String>>()
        },
        &Distro::Debian => {
            results
                .into_iter()
                .filter(|package| !package.debian.is_empty())
                .map(|package| package.debian)
                .collect::<Vec<String>>()
        },
        &Distro::Mint => {
            results
                .into_iter()
                .filter(|package| !package.mint.is_empty())
                .map(|package| package.ubuntu)
                .collect::<Vec<String>>()
        },
        &Distro::Freebsd => {
            results
                .into_iter()
                .filter(|package| !package.freebsd.is_empty())
                .map(|package| package.freebsd)
                .collect::<Vec<String>>()
        },
        _ => Vec::new(),
    }
}
