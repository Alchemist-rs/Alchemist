/// Struct for a row in the Alchemist package table
#[derive(PartialEq,PartialOrd,Eq,Ord,Hash,Debug,Queryable)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub arch: String,
    pub ubuntu: String,
    pub mint: String,
    pub debian: String,
    pub gentoo: String,
    pub void: String,
    pub mac: String,
    pub freebsd: String,
    pub netbsd: String,
    pub openbsd: String,
}

impl Package {
    /// Returns a Package with no data for instances of error handling
    /// when connecting to the db and finding nothing
    pub fn empty() -> Package {
        Package {
            id: 0,
            name: String::from(""),
            arch: String::from(""),
            ubuntu: String::from(""),
            mint: String::from(""),
            debian: String::from(""),
            gentoo: String::from(""),
            void: String::from(""),
            mac: String::from(""),
            freebsd: String::from(""),
            netbsd: String::from(""),
            openbsd: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Distro {
    Arch,
    Ubuntu,
    Mint,
    Debian,
    Gentoo,
    Void,
    Mac,
    Freebsd,
    Netbsd,
    Openbsd,
}
