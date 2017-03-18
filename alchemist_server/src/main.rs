#![deny(missing_docs, missing_debug_implementations,
        missing_copy_implementations, trivial_casts, trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]
#![cfg_attr(feature = "dev", allow(unstable_features))]

// bits for feature gating the default use of clippy, no need to make it default.
#![cfg_attr(feature = "dev", plugin(clippy))]
#![cfg_attr(feature = "dev", plugin)]

// bits for rocket
#![feature(plugin)]
#![plugin(rocket_codegen)]

/// #Alchemist
/// Alchemist_Server is a API server for determing what package should be installed for a distro
/// based on input. Made for Alchemist, however it should be availibe to use to anyone/script.
///
/// The API can be reached at ``https://alchemist.rs/api/v0/`` (you can replace this url if you
/// are developing this locally or running this server elsewhere)
///
/// ## Endpoints
/// ### Index
/// ``GET /``
/// returns the API gretting response.
/// Example Response:
///
/// ```json
/// {
/// "msg": "Alchemist public API server",
/// "version": "0.0.1"
/// }
///
/// ### Version
/// ``GET /version``
/// Returns Alchemist server & client versions
/// Example Response:
/// ```json
/// {
/// "client": "0.0.4"
/// "server": "0.0.1"
/// }
/// ```
/// Addtionly you can GET '''/version/client''' & ```/version/server```
/// respectivly to get only one or the other in the response.
///
/// ### Package
/// ``POST /package``
/// Takes a JSON POST with four variables ``package``, ``distro``, ``client``, ``client_version``.
/// ``Package`` should be what the user specified for the package to search for.
/// ``distro`` is the distro that the user is currently using.
/// ``client``[ ``name`` & ``version`` ] are for current client accessing the API, used for analics & to
/// check if alchemist is up to date, they are needed, however you can put whatever you want in
/// there (doesn't need to be capital) however we'd prefer if you used real client names based on
/// your application.
/// Example POST to ``/package``:
/// ```json
/// {
/// "package": ["lib-gcc"],
/// "distro": "Arch",
/// "client": {
///     "name": "alchemist",
///     "version": "0.0.4"
///     }
/// }
/// ```
/// Returns the package to be installed for the distro supplied in the POST request.
/// ```json
/// {
/// "package": "libgcc"
/// }
/// ```

extern crate rocket;
extern crate serde;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate alchemist_server;
extern crate diesel;

use self::alchemist_server::*;
use self::alchemist_server::models::*;
use rocket_contrib::{JSON, Value};
use rocket::State;
use std::collections::HashSet;
use std::string::String;

// The serde structs used for Serializing & Deserializing JSON structures.
#[feature(serde_derive)]
#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
    msg: String,
    version: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Client_Version {
    client: String
}
#[derive(Serialize, Deserialize, Debug)]
struct Server_Version {
    server: String
}
#[derive(Serialize, Deserialize, Debug)]
struct Current_Version {
    current: bool
}
#[derive(Serialize, Deserialize, Debug)]
struct Package {
    package: Vec<String>
}
#[derive(Serialize, Deserialize, Debug)]
struct Package_Incoming {
	package: Vec<String>,
    distro: String,
    client: Client
}
#[derive(Serialize, Deserialize, Debug)]
struct Client {
    name: String,
    version: String
}

// Index response.
#[get("/")]
fn index() -> JSON<Greeting> {
    JSON(Greeting {
        // Server Hello message
        msg: "Alchemist public API server.".to_string()
        // This is coming from cargo rather than crates.io since this is supposed to return the
        // current running version of the server.
        version: env!("CARGO_PKG_VERSION").to_string()
    })
}

// Get the current version of the client/server
#[get("/version/<number>")]
fn version(number: &str) -> JSON<Version> {
    if number == client {
        JSON(Client_Version {
            client: "0.0.4".to_string()
        })
    } else if number == server {
        JSON(Server_Version {
            server: "0.0.1".to_string()
        })
    } else {
        JSON(Client_Version {
            client: "0.0.4"
        }, Server_Version {
            server: "0.0.1"
        })
    }
}

// The package post, allows you to POST and get the correct package for your picture
#[post("/package", format = "application/json", data = "<message>")]
fn package(pac: JSON<Package_Incoming>) -> JSON<Package> {
    let mut hashset_pac = HashSet::new();
    let v = pac.package;

    for p in v {
        hashset_pac.insert(p);
    }

    let distro_packages = convert_to_distro(hashset_pac, pac.distro);

    if !distro_packages.is_empty() {
        let alchem = Vec::new();

        for packa in distro_packages {
            alchem.push(packa);
        }

        JSON(Package {
            package: alchem
        })
    } else {
        let alchem_fail = Vec::new().push("none");
        JSON(Package {
            package: alchem_fail
        })
    }
}

// 404 page
#[error(404)]
fn not_found(request: &Request) -> content::HTML<String> {
    let html = match request.content_type() {
        Some(ref ct) if !ct.is_json() => {
            format!("<p>This server only supports JSON requests, not '{}'.</p>", ct)
        }
        _ => json!({"error": format!("Sorry, '{}' is an invalid path! Take a look at the docs located at https://alchemist.rs for all current valid paths.</p>", request.uri())})
    };

    content::HTML(html)
}

// Put rocket in its own thing.
fn rocket -> rocket::Rocket {
    rocket::ignite()
        .mount("/api/v0/", routes![index, version, package])
        .catch(errors![not_found])
}

// Programs main, launches the rocket server.
fn main() {
    rocket().launch();
}
