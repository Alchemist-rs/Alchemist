#[macro_export]
/// Sets up and runs the code for various package commands rather than writing it all out
/// manually each time.
macro_rules! package_command {
    ($pkg:expr, $flag:expr) => {{
        use std::process::Command;
        let child = Command::new($pkg)
                        .arg($flag)
                        .spawn()
                        .chain_err(|| "Failed to execute process")?;

        let output = child.wait_with_output()
                        .chain_err(|| "Failed to wait on child process")?;

        if output.status.success() {
            println!("{}", String::from_utf8(output.stdout)
                                .chain_err(|| "Unable to convert child process stdout to UTF-8")?);
        } else {
            println!("{}", String::from_utf8(output.stderr)
                                .chain_err(|| "Unable to convert child process stderr to UTF-8")?);
        }

        Ok(())
    }};
    ($pkg:expr, $flag:expr, $packages:expr) => {{
        use std::process::Command;
        let child = Command::new($pkg)
                        .arg($flag)
                        .args($packages.into_iter()
                                .collect::<Vec<&str>>()
                                .as_slice())
                        .spawn()
                        .chain_err(|| "Failed to execute process")?;

        let output = child.wait_with_output()
                        .chain_err(|| "Failed to wait on child process")?;

        if output.status.success() {
            println!("{}", String::from_utf8(output.stdout)
                                .chain_err(|| "Unable to convert child process stdout to UTF-8")?);
        } else {
            println!("{}", String::from_utf8(output.stderr)
                                .chain_err(|| "Unable to convert child process stderr to UTF-8")?);
        }

        Ok(())
    }};
}

#[macro_export]
/// Sets up and runs the code for various package commands rather than writing it all out
/// manually each time. Does not return a result in the case where package_command is needed
/// twice
macro_rules! package_command_no_return {
    ($pkg:expr, $flag:expr) => {{
        use std::process::Command;
        let child = Command::new($pkg)
                        .arg($flag)
                        .spawn()
                        .chain_err(|| "Failed to execute process")?;

        let output = child.wait_with_output()
                        .chain_err(|| "Failed to wait on child process")?;

        if output.status.success() {
            println!("{}", String::from_utf8(output.stdout)
                                .chain_err(|| "Unable to convert child process stdout to UTF-8")?);
        } else {
            println!("{}", String::from_utf8(output.stderr)
                                .chain_err(|| "Unable to convert child process stderr to UTF-8")?);
        }
    }};
    ($pkg:expr, $flag:expr, $packages:expr) => {{
        use std::process::Command;
        let child = Command::new($pkg)
                        .arg($flag)
                        .args($packages.into_iter()
                                .collect::<Vec<&str>>()
                                .as_slice())
                        .spawn()
                        .chain_err(|| "Failed to execute process")?;

        let output = child.wait_with_output()
                        .chain_err(|| "Failed to wait on child process")?;

        if output.status.success() {
            println!("{}", String::from_utf8(output.stdout)
                                .chain_err(|| "Unable to convert child process stdout to UTF-8")?);
        } else {
            println!("{}", String::from_utf8(output.stderr)
                                .chain_err(|| "Unable to convert child process stderr to UTF-8")?);
        }
    }};
}
