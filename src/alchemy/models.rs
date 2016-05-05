#[derive(Queryable, Debug)]
pub struct Package {
    pub id: i32,
    pub arch: String,
    pub aur: String,
    pub ubuntu: String,
    pub ubuntu_dev: String
}
