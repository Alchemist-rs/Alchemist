use std::fs;

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

///Returns what Distribution the user is using
///If no possible match is found return a None
pub fn which_distro() -> Option<Distro> {

    let arch= fs::metadata("/etc/arch-release");

    //Better way to do this?
    if arch.is_ok() && arch.unwrap().is_file() {
           return Some(Distro::Arch);
    }

    //No distro was found to match
    None
}
