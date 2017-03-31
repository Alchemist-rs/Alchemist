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

// The package post, allows you to POST and get the correct package for your picture
#[post("/package", format = "application/json", data = "<pkgincoming>")]
pub fn package(pkgincoming: Result<JSON<PkgIncoming>, SerdeError>)
               -> Result<JSON<Package>, String> {
    match pkgincoming {
        Err(why) => {
            Err(json!({
                          "error": format!("There was an error parsing the JSON: {:?}", why),
                          "note": "You probably messed up the syntax, check the docs @ https://alchemist.rs to ensure your sending a valid JSON request."
                      })
                        .to_string())
        }
        Ok(pkg_incoming) => {
            let mut distro_packages = convert_to_distro(pkg_incoming.package.clone(),
                                                        &pkg_incoming.distro);

            if !distro_packages.is_empty() {
                Ok(JSON(Package { package: distro_packages }))
            } else {
                distro_packages.push("none".to_string());
                Ok(JSON(Package { package: distro_packages }))
            }
        }
    }
}

// 404 page
#[error(404)]
pub fn not_found(request: &Request) -> content::HTML<String> {
    // So this is a bit confusing so I'm going to do my best to explain this.
    // request.content_type() will return as a option, with Some if there was a header in request,
    // and None if there wasn't. This means that if a client doesn't specify a header then it will
    // go to _.
    let html = match request.content_type() {
        Some(ref content) if !content.is_json() => {
            format!("<p>This server only supports JSON requests, not '{}'.</p>",
                    content)
        }
        _ => {
            json!({
                      "error": format!("Sorry, '{}' is an invalid path!", request.uri()),
                      "note": "Try the docs @ https://alchemist.rs for valid paths."
                  })
                    .to_string()
        }
    };

    content::HTML(html)
}
