extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use models::Packages;
use std::env;
use std::string::String;
use std::collections::HashSet;


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
/// ```
/// let mut packages: HashSet<&str> = HashSet::new()
/// packages.insert("sudo")
/// packages.insert("postgresql")
/// let queryed = pack_query(packages);
/// ```
///
pub fn pack_query(input_packages: HashSet<String>) -> HashSet<Packages> {
    use schema::packages::dsl::*;

    let connection = establish_connection();

    let mut output: HashSet<Packages> = HashSet::new();

    for i in input_packages {
        // While this might look like O(n^2) complexity
        // it's more closer to O(n) since most querys only
        // return one or 2 results really. Still this seemed
        // to be the only way to implement it well
        let results = packages.filter(arch.eq(&i)
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
            .get_results::<Packages>(&connection)
            .unwrap_or(vec![Packages::empty()]);
        for j in results {
            output.insert(j);
        }
    }

    output
}

/// Convert package names from other distros to the one being run by the user currently
fn convert_to_distro(input_packages: HashSet<String>, distro: String) -> HashSet<String> {
    let results = pack_query(input_packages);
    let mut pac_converted: HashSet<String> = HashSet::new();

    // All querys will either be a string or '' in the db allowing us to
    // use is_empty()
    // Finds out what distro is used and inserts the proper conversions
    // into the HashSet to be returned from the function
    match distro {
        Ubuntu => {
            for i in results {
                if !i.Ubuntu.is_empty() {
                    pac_converted.insert(i.Ubuntu);
                }
            }
        }
        Void => {
            for i in results {
                if !i.Void.is_empty() {
                    pac_converted.insert(i.Void);
                }
            }
        }
        Debian => {
            for i in results {
                if !i.Debian.is_empty() {
                    pac_converted.insert(i.Debian);
                }
            }
        }
        Mint => {
            for i in results {
                if !i.Mint.is_empty() {
                    pac_converted.insert(i.Mint);
                }
            }
        }
        FreeBSD => {
            for i in results {
                if !i.FreeBSD.is_empty() {
                    pac_converted.insert(i.FreeBSD);
                }
            }
        }
        _ => {
            pac_converted.insert(String::new());
        }
    }

    pac_converted
}
