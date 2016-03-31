use std::fs;

pub fn has_sudo() -> bool {

    let sudo = fs::metadata("/usr/bin/sudo");

    if sudo.is_ok() {
        return sudo.expect("Should have been okay")
                   .is_file();
    }

    false
}
