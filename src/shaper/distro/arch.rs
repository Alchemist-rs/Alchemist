extern crate diesel;

use std::process::{Command};
use super::super::db;
use super::super::models::Package;
use diesel::prelude::*;
use diesel::expression::dsl::*;

pub fn pac_install(packages: Vec<&str>) {
    let arch_packages = convert_to_arch(packages);
    let mut child = match Command::new("pacman")
            .arg("-S")
            .args(arch_packages.as_slice())
            .spawn()
    {
        Ok(child) => child,
        Err(e)    => panic!("Failed to execute child: {}",e),
    };
    let _unused = child.wait();
}

fn convert_to_arch(input_packages: Vec<&str>) -> Vec<String> {
    use super::super::schema::packages::dsl::*;
    let connection = db::establish_connection();
    let results = packages
        .filter(
            arch.eq(any(&input_packages))
            .or(ubuntu.eq(any(&input_packages)))
        )
        .load::<Package>(&connection)
        .expect("Error loading posts");

    let mut converted: Vec<String> = Vec::new();
    for i in results {
        //All querys will either be a string or '' in the db
        if !i.arch.is_empty() {
            converted.push(i.arch);
        }
    }
    converted
}
