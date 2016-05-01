use std::fs;

pub fn has_sudo() -> bool {

    let sudo = fs::metadata("/usr/bin/sudo");

    if sudo.is_ok() && sudo.unwrap().is_file() {
        return true
    }

    false
}
