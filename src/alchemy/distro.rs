use std::fs;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::string::String;
use std::convert::AsRef;

#[derive(Debug)]
///Enum used to represent Distribution used
pub enum Distro {
    //Linux Distributions
    Arch,
    Ubuntu,
    Mint,
    Debian,
    Gentoo,

    //Berklee Unix Distributions
    Mac,
    FreeBSD,
    NetBSD,
    OpenBSD
}

/// Returns what Distribution the user is using
/// If no possible match is found return a None
///
/// #Examples
///
/// ```
/// let distro = which_distro();
/// println!("{:?}",distro);
/// ```
///
/// Currently only returns if the user is using
/// Arch Linux and no other distribution
///
pub fn which_distro() -> Option<Distro> {

    let distro = fs::metadata("/proc/version");
    let arch = fs::metadata("/etc/arch-release")
    let mut f = try!(File::open("version"));
    let mut buffer = String::new();

    try!(f.read_to_string(&mut buffer));
    match buffer.as_ref() {
        "void" => return Some(Distro::Void);
        "arch" => return Some(Distro::Arch);
    }


    if arch.is_ok() && distro.unwrap().is_file() {
        return Some(Distro::Arch)
    }

    //No distro was found to match
    None
}
