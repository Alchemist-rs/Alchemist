use db::*;
use models::Distro;
use rocket::Request;
use rocket::response::content;
use rocket_contrib::JSON;
use rocket_contrib::SerdeError;
use std::string::String;

// The serde structs used for Serializing & Deserializing JSON structures.
#[derive(Serialize, Deserialize, Debug)]
pub struct Greeting {
    msg: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    package: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PkgIncoming {
    package: Vec<String>,
    distro: Distro,
    client: Client,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    name: String,
    version: String,
}


// Index response.
#[get("/")]
pub fn index() -> JSON<Greeting> {
    JSON(Greeting {
             // Server Hello message
             msg: "Alchemist public API server.".to_string(),
         })
}

#[get("/pkg_test")]
pub fn pkg_test() -> JSON<PkgIncoming> {
    let mut pac = Vec::new();
    pac.push("sudo".to_string());

    JSON(PkgIncoming {
             package: pac,
             distro: Distro::Arch,
             client: Client {
                 name: "Alchemist".to_string(),
                 version: "0.0.4".to_string(),
             },
         })
}

// The package post, allows you to POST and get the correct package for your picture
#[post("/package", format = "application/json", data = "<pkgincoming>")]
pub fn package(pkgincoming: JSON<PkgIncoming>) -> JSON<Package> {
    let mut distro_packages = convert_to_distro(pkgincoming.package.clone(), &pkgincoming.distro);

    if !distro_packages.is_empty() {
        JSON(Package { package: distro_packages })
    } else {
        distro_packages.push("none".to_string());
        JSON(Package { package: distro_packages })
    }
}

// 404 page
#[error(404)]
pub fn not_found(request: &Request) -> content::HTML<String> {
    let html = match request.content_type() {
        Some(ref content) if !content.is_json() => {
            format!("<p>This server only supports JSON requests, not '{}'.</p>",
                    content)
        }
        _ => {
            json!({
                      "error": format!("Sorry, '{}' is an invalid path! \
                          Try the docs @ https://alchemist.rs for valid paths.",
                          request.uri())
                  })
                    .to_string()
        }
    };

    content::HTML(html)
}
