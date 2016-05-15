#[derive(Queryable,PartialEq,PartialOrd,Eq,Ord,Hash,Debug)]
/// Struct for a row in the Alchemist package table
pub struct Package {
    /// Row Number makes it easy for migration
    pub id: i32,
    /// Arch Linux Package
    pub arch: String,
    /// Arch Linux AUR Package
    pub aur: String,
    /// Debian Package
    pub debian: String,
    /// Void Linux Package
    pub void: String,
    /// Ubuntu Binary Packages
    pub ubuntu: String,
    /// Ubuntu Development Header Packages
    pub ubuntu_dev: String
}

impl Package {
    ///Returns a Package with no data for instances of error handling
    ///when connecting to the db and finding nothing
    pub fn empty() -> Package {
        Package {
            id: 0,
            arch: String::from(""),
            aur : String::from(""),
            debian: String::from(""),
            void: String::from(""),
            ubuntu: String::from(""),
            ubuntu_dev: String::from(""),
        }
    }
}
