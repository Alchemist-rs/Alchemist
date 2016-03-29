extern crate shaper;
use shaper::distro::*;
use shaper::distro::common::{Distro,which_distro};
use std::process::exit;

fn main() {
    let opt_dist = which_distro();
    if opt_dist.is_none() {
        println!("Your distribution is not supported");
        exit(0);
    }

    let dist = opt_dist.expect("None Distro was not handled");
    match dist {
        Distro::Arch    => println!("Arch Linux"),
        Distro::Ubuntu  => println!("Ubuntu"),
        Distro::Mint    => println!("Mint"),
        Distro::Debian  => println!("Debian"),
        Distro::Gentoo  => println!("Gentoo"),
        Distro::Mac     => println!("Mac"),
        Distro::FreeBSD => println!("FreeBSD"),
        Distro::NetBSD  => println!("NetBSD"),
        Distro::OpenBSD => println!("OpenBSD"),
    }
}
