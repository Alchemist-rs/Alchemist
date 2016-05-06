use std::fs;

/// Checks if the user has sudo installed on their system
///
/// ```
/// if has_sudo() {
///     println!("They have sudo");
/// } else {
///     println!("They do not have sudo");
/// }
/// ```
///
pub fn has_sudo() -> bool {

    let sudo = fs::metadata("/usr/bin/sudo");

    if sudo.is_ok() && sudo.unwrap().is_file() {
        return true
    }

    false
}

/// Checks if the user is running as root
///
/// #Examples
///
/// ```
/// if is_root() {
///     println!("All your base are belong to us");
/// } else {
///     println!("Do you even root bro?");
/// }
/// ```
///
/// Currently this does not work
///
pub fn is_root() -> bool {
    //stub
    false
}
