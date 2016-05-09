#[derive(Queryable, Debug)]
/// Struct for a row in the Alchemist package table
pub struct Package {
    /// Row Number makes it easy for migration
    pub id: i32,
    /// Arch Linux Package
    pub arch: String,
    /// Arch Linux AUR Package
    pub aur: String,
    /// Ubuntu Binary Packages
    pub ubuntu: String,
    /// Ubuntu Development Header Packages
    pub ubuntu_dev: String
}
