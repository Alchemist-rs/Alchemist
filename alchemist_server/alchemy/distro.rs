use std::fs;
use std::io::prelude::*;
use std::fs::File;
use std::string::String;

#[derive(Debug)]
/// Enum used to represent Distribution used
pub enum Distro {
    // Linux Distributions
    Arch,
    Ubuntu,
    Mint,
    Debian,
    Gentoo,
    Void,

    // Berklee Unix Distributions
    Mac,
    FreeBSD,
    NetBSD,
    OpenBSD,
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

    // Open Proc and read into string the contents of it to be
    // be checked below
    let mut f = File::open("/proc/version")
        .unwrap_or_else(|e| panic!("Failed to open /proc/version: {}", e));
    let mut buffer = String::new();
    let _unused = f.read_to_string(&mut buffer);

    // Check for distros in Alphabetical order
    if buffer.contains("arch") {
        return Some(Distro::Arch);
    } else if buffer.contains("ubuntu") {
        return Some(Distro::Ubuntu);
    } else if buffer.contains("void") {
        return Some(Distro::Void);
    } else if buffer.contains("debian") {
        return Some(Distro::Debian);
    } else if buffer.contains("mint") {
        return Some(Distro::Mint);
    }

    // Checks for FreeBSD after the previous method
    // tries using /proc/version
    let freebsd = fs::metadata("/bin/freebsd-version");
    if freebsd.is_ok() && freebsd.unwrap().is_file() {
        return Some(Distro::FreeBSD);
    }
    // Checks for Arch differently but only after the
    // previous method checks for FreeBSD
    let arch = fs::metadata("/etc/arch-release");
    if arch.is_ok() && arch.unwrap().is_file() {
        return Some(Distro::Arch);
    }
    // Check for Debian differently but only after the
    // previous method has tried by using /etc/arch-release
    let debian = fs::metadata("/etc/debian_version");
    if debian.is_ok() && debian.unwrap().is_file() {
        return Some(Distro::Debian);
    }
    // No distro was found to match
    None
}

pub trait Distribution {

}
