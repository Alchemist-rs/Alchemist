// We need this for error-chain
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate clap;

// We import macros first so we have
// access in all files
#[macro_use]
mod macros;

mod pkg_manager;

// This sets up error-chain
mod errors {
    error_chain!{}
}


use clap::{App, Arg};
use errors::*;
use pkg_manager::*;
use std::collections::HashSet;
use std::process::exit;

fn main() {

    // Handles errors passed up the execution via error-chain
    if let Err(ref e) = run() {
        // Print out the error
        println!("error: {}", e);

        // If more than one error exists print them out
        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // If there is a backtrace print it out
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        // Code failed give a non zero exit code
        exit(1);
    }
}

/// This function is what drives the code to check for the package
/// manager and then distro for querying of packages.
fn run() -> Result<()> {

    let mut app = App::new("Alchemist")
        .version("0.4.0")
        .author("Michael Gattozzi <mgattozzi@gmail.com>")
        .about("Unix Platform Agnostic Installer")
        .arg(Arg::with_name("install")
                 .short("i")
                 .takes_value(true)
                 .multiple(true)
                 .help("Install the given programs"))
        .arg(Arg::with_name("refresh")
                 .short("r")
                 .long("refresh")
                 .help("Refresh package list with newest version")
                 .takes_value(false)
                 .conflicts_with("upgrade"))
        .arg(Arg::with_name("upgrade")
                 .short("u")
                 .long("upgrade")
                 .help("Upgrade packages to newest version")
                 .takes_value(false)
                 .conflicts_with("install"));

    let matches = app.clone().get_matches();

    if matches.args.is_empty() {

        app.print_help()
            .chain_err(|| "Failed to print help message")?;

        // If you don't do this then the string prints out as if it isn't
        // terminated in the console for some reason.
        println!();

    } else {

        // Determine package manager of User
        let mgr = which_manager()?;

        // It should be noted that since we don't have impl Trait in stable Rust
        // yet we need this enum to determine the correct one to use. Eventually
        // mgr will just be an impl Trait returned item that can just call
        // refresh, upgrade, or install directly. (i.e. mgr.refresh()? or
        // mgr.upgrade()?)

        if matches.is_present("refresh") {
            match mgr {
                Manager::AptGet => AptGet.refresh()?,
                Manager::Pacman => Pacman.refresh()?,
                Manager::Pkg => Pkg.refresh()?,
                Manager::HomeBrew => HomeBrew.refresh()?,
                Manager::Xbps => Xbps.refresh()?,
                Manager::Aura => Aura.refresh()?,
                Manager::Pacaur => Pacaur.refresh()?,
                Manager::Packer => Packer.refresh()?,
                Manager::Yaourt => Yaourt.refresh()?,
            }
        } else if matches.is_present("upgrade") {
            match mgr {
                Manager::AptGet => AptGet.upgrade()?,
                Manager::Pacman => Pacman.upgrade()?,
                Manager::Pkg => Pkg.upgrade()?,
                Manager::HomeBrew => HomeBrew.upgrade()?,
                Manager::Xbps => Xbps.upgrade()?,
                Manager::Aura => Aura.upgrade()?,
                Manager::Pacaur => Pacaur.upgrade()?,
                Manager::Packer => Packer.upgrade()?,
                Manager::Yaourt => Yaourt.upgrade()?,
            }
        }

        if let Some(p) = matches.values_of("install") {
            let package_inputs: HashSet<&str> = p.collect();

            // Small little easter egg
            if package_inputs.contains("pb") {
                bail!("Looks like you're trying to turn lead into gold. \
                    That's not how this program works.");
            }

            match mgr {
                Manager::AptGet => AptGet.install(package_inputs)?,
                Manager::Pacman => Pacman.install(package_inputs)?,
                Manager::Pkg => Pkg.install(package_inputs)?,
                Manager::HomeBrew => HomeBrew.install(package_inputs)?,
                Manager::Xbps => Xbps.install(package_inputs)?,
                Manager::Aura => Aura.install(package_inputs)?,
                Manager::Pacaur => Pacaur.install(package_inputs)?,
                Manager::Packer => Packer.install(package_inputs)?,
                Manager::Yaourt => Yaourt.install(package_inputs)?,
            }
        }
    }

    Ok(())
}
