use std::fs;

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

    let arch= fs::metadata("/etc/arch-release");

    if arch.is_ok() && arch.unwrap().is_file() {
           return Some(Distro::Arch);
    }

    //No distro was found to match
    None
}
