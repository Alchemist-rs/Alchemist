use std::process::{Command};
pub fn pac_install(packages: Vec<&str>) {
    let mut child = match Command::new("pacman")
            .arg("-S")
            .args(packages.as_slice())
            .spawn()
    {
        Ok(child) => child,
        Err(e)    => panic!("Failed to execute child: {}",e),
    };
    let _unused = child.wait();
}
