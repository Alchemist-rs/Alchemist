#![deny(missing_debug_implementations,
        missing_copy_implementations, trivial_casts, trivial_numeric_casts,
       unsafe_code, unused_import_braces, unused_qualifications)]
// bits for feature gating the default use of clippy, no need to make it default.
#![cfg_attr(feature = "dev", plugin(clippy))]
#![cfg_attr(feature = "dev", plugin)]

// bits for rocket
#![feature(plugin, custom_attribute, attr_literals)]
#![plugin(rocket_codegen)]

//! Alchemist_Server
//! Alchemist_Server is a API server for determing what package should be installed for a distro
//! based on input. Made for Alchemist, however it should be availibe to use to anyone/script.
//!
//! The API can be reached at ``https://alchemist.rs/api/v0/`` (you can replace this url if you
//! are developing this locally or running this server elsewhere)
//!
//! ## Endpoints
//! ### Index
//! ``GET /``
//! returns the API gretting response.
//! Example Response:
//!
//! ```json
//! {
//! "msg": "Alchemist public API server",
//! }
//!
//! ### Package
//! ``POST /package``
//! Takes a JSON POST with four variables ``package``, ``distro``, ``client``, ``client_version``.
//! ``Package`` should be what the user specified for the package to search for.
//! ``distro`` is the distro that the user is currently using.
//! ``client``[ ``name`` & ``version`` ] are for current client accessing the API, used for analytics & to
//! check if alchemist is up to date, they are needed, however you can put whatever you want in
//! there (doesn't need to be capital) however we'd prefer if you used real client names based on
//! your application.
//! Example POST to ``/package``:
//! ```json
//! {
//! "package": ["lib-gcc"],
//! "distro": "Arch",
//! "client": {
//!     "name": "alchemist",
//!     "version": "0.0.4"
//!     }
//! }
//! ```
//!
//! #### Package Return Response
//! Returns the package to be installed for the distro supplied in the POST request.
//! ```json
//! {
//! "package": ["libgcc"]
//! }
//! ```

extern crate rocket;
extern crate rocket_contrib;
extern crate alchemist_server;

use self::alchemist_server::server_routes::*;

// Put rocket in its own thing.
pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/api/v0/", routes![index, package, pkg_test]).catch(errors![not_found])
}

// Programs main, launches the rocket server.
fn main() {
    rocket().launch();
}
