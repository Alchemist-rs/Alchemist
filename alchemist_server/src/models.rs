/// Struct for a row in the Alchemist package table
#[derive(PartialEq,PartialOrd,Eq,Ord,Hash,Debug,Queryable)]
pub struct Packages {
    pub id: i32,
    pub Arch: String,
    pub Ubuntu: String,
    pub Mint: String,
    pub Debian: String,
    pub Gentoo: String,
    pub Void: String,
    pub Mac: String,
    pub FreeBSD: String,
    pub NetBSD: String,
    pub OpenBSD: String,
}

impl Packages {
    /// Returns a Package with no data for instances of error handling
    /// when connecting to the db and finding nothing
    pub fn empty() -> Packages {
        Packages {
            id: 0,
            Arch: String::from(""),
            Ubuntu: String::from(""),
            Mint: String::from(""),
            Debian: String::from(""),
            Gentoo: String::from(""),
            Void: String::from(""),
            Mac: String::from(""),
            FreeBSD: String::from(""),
            NetBSD: String::from(""),
            OpenBSD: String::from(""),
        }
    }
}
