extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use models::Package;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn pack_query(input_packages: Vec<&str>) -> Vec<Package> {
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
