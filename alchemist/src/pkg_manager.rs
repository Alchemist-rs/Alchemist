use std::process::Command;
use std::collections::HashSet;
use errors::*;

/// Run the which command to see if the package manager is there
fn which(mgr: &str) -> Result<bool> {
    Ok(Command::new("which")
        .arg(mgr)
        .output()
        .chain_err(|| "Failed to execute process")?
        .status
        .success())
}

/// Returns what Distribution the user is using
pub fn which_manager() -> Result<Manager> {
    if which("apt-get")? {
        Ok(Manager::AptGet)
    } else if which("pkg")? {
        Ok(Manager::Pkg)
    } else if which("homebrew")? {
        Ok(Manager::HomeBrew)
    } else if which("xbps-install")? {
        Ok(Manager::Xbps)
    } else if which("aura")? {
        Ok(Manager::Aura)
    } else if which("pacaur")? {
        Ok(Manager::Pacaur)
    } else if which("packer")? {
        Ok(Manager::Packer)
    } else if which("yaourt")? {
        Ok(Manager::Yaourt)
    // Check last because a helper might be available first
    } else if which("pacman")? {
        Ok(Manager::Pacman)
    // No distro was found to match
    } else {
        bail!("Your distribution is not supported.");
    }
}

// This is a work around until impl Trait gets stabilized.
// Then we could pass back the structs below directly
/// Enum representing all possible package managers
pub enum Manager {
    AptGet,
    Pacman,
    Pkg,
    HomeBrew,
    Xbps,
    Aura,
    Pacaur,
    Packer,
    Yaourt,
}

// Official Package Managers
/// Representation of apt-get package manager
pub struct AptGet;
/// Representation of pacman package manager
pub struct Pacman;
/// Representation of pkg package manager
pub struct Pkg;
/// Representation of homebrew package manager
pub struct HomeBrew;
/// Representation of xbps package manager
pub struct Xbps;

// AUR Helper
/// Uses Aura to install packages from the AUR
pub struct Aura;
/// Uses Pacaur to install packages from the AUR
pub struct Pacaur;
/// Uses Packer to install packages from the AUR
pub struct Packer;
/// Uses Yaourt to install packages from the AUR
pub struct Yaourt;


/// Functions all Package Managers must implement
pub trait PackageManager {
    fn refresh(&self) -> Result<()>;
    fn upgrade(&self) -> Result<()>;
    fn install(&self, pkg: HashSet<&str>) -> Result<()>;
}

/// Implementation of commands for apt-get
impl PackageManager for AptGet {
    fn refresh(&self) -> Result<()> {
        package_command!("apt-get", "update")
    }

    fn upgrade(&self) -> Result<()> {
        package_command!("apt-get", "upgrade")
    }

    fn install(&self, pkg: HashSet<&str>) -> Result<()> {
        package_command!("apt-get", "install", pkg)
    }
}

/// Implementation of commands for pacman
impl PackageManager for Pacman {
    fn refresh(&self) -> Result<()> {
        package_command!("pacman", "-Syy")
    }

    fn upgrade(&self) -> Result<()> {
        package_command!("pacman", "-Syyu")
    }

    fn install(&self, pkg: HashSet<&str>) -> Result<()> {
        package_command!("pacman", "-S", pkg)
    }
}

/// Implementation of commands for Pkg
impl PackageManager for Pkg {
    fn refresh(&self) -> Result<()> {
        package_command!("pkg", "update")
    }

    fn upgrade(&self) -> Result<()> {
       // pkg docs unclear
        package_command!("pkg", "update")
    }

    fn install(&self, pkg: HashSet<&str>) -> Result<()> {
        package_command!("pkg", "install", pkg)
    }
}

/// Implementation of commands for Homebrew
impl PackageManager for HomeBrew {
    fn refresh(&self) -> Result<()> {
        package_command!("brew", "update")
    }

    fn upgrade(&self) -> Result<()> {
        package_command_no_return!("brew", "update");
        package_command!("brew", "upgrade")
    }

    fn install(&self, pkg: HashSet<&str>) -> Result<()> {
        package_command!("brew", "install", pkg)
    }
}

/// Implementation of commands for Xbps
impl PackageManager for Xbps {
    fn refresh(&self) -> Result<()> {
        package_command!("xbps-install", "-S")
    }

    fn upgrade(&self) -> Result<()> {
        package_command!("xbps-install", "-Su")
    }

    fn install(&self, pkg: HashSet<&str>) -> Result<()> {
        package_command!("xbps-install", "", pkg)
    }
}

/// Implementation of commands for Aura
impl PackageManager for Aura {
    fn refresh(&self) -> Result<()> {
        package_command!("aura", "-Syy")
    }

    fn upgrade(&self) -> Result<()> {
        package_command_no_return!("aura", "-Syyu");
        package_command!("aura", "-Au")
    }

    fn install(&self, pkg: HashSet<&str>) -> Result<()> {
        // Find a way to split packages up
        package_command_no_return!("aura", "-A", pkg.clone());
        package_command!("aura", "-S", pkg)
    }

}

/// Implementation of commands for Pacaur
impl PackageManager for Pacaur {
    fn refresh(&self) -> Result<()> {
        package_command!("pacaur","-Syy")
    }

    fn upgrade(&self) -> Result<()> {
        package_command!("pacaur","-Syyu")
    }

    fn install(&self, pkg: HashSet<&str>) -> Result<()> {
        package_command!("pacaur","-S", pkg)
    }

}

/// Implementation of commands for Packer
impl PackageManager for Packer {
    fn refresh(&self) -> Result<()> {
        package_command!("packer","-Syy")
    }

    fn upgrade(&self) -> Result<()> {
        package_command!("packer","-Syyu")
    }

    fn install(&self, pkg: HashSet<&str>) -> Result<()> {
        package_command!("packer","-S", pkg)
    }

}


/// Implementation of commands for Yaourt
impl PackageManager for Yaourt {
    fn refresh(&self) -> Result<()> {
        package_command!("yaourt","-Syy")
    }

    fn upgrade(&self) -> Result<()> {
        package_command!("yaourt","-Syyu --aur")
    }

    fn install(&self, pkg: HashSet<&str>) -> Result<()> {
        package_command!("yaourt","", pkg)
    }

}
