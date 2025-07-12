use anyhow::{Context, Result};
use pathman::{PathmanError, UpdateType, prepend_to_path};
use std::{
    fs::{create_dir_all, rename},
    path::PathBuf,
};

use crate::platform::Platform;

pub struct Installer {
    platform: Platform,
    install_dir: PathBuf,
}

impl Installer {
    /// Creates a new installer
    pub fn new(install_dir: PathBuf) -> Self {
        Installer {
            platform: Platform::detect(),
            install_dir,
        }
    }

    /// Installs the Biome binary to the specified directory
    ///
    /// This functions ensures that the installation directory exists,
    /// and moves the binary at the specified path to the installation
    /// directory. It then ensures that the binary is executable.
    pub fn install(&self, temp_bin: PathBuf) -> Result<PathBuf> {
        // Ensure the installation directory exists
        create_dir_all(&self.install_dir).context("Could not create the installation directory")?;

        // Build the destination path
        let bin = self
            .install_dir
            .join(format!("biome{}", self.platform.extension));

        // Move the binary to the installation directory
        rename(&temp_bin, &bin)
            .context("Failed to move the binary to the installation directory")?;

        // Make the binary executable
        self.make_executable(&bin)
            .context("Failed to make the binary executable")?;

        Ok(bin)
    }

    /// Prepends the installation directory to the PATH environment variable
    ///
    /// This function checks if the installation directory is already in the PATH,
    /// and if not, it prepends it to the PATH using a platform specific method.
    ///
    /// On Unix-like systems, it adds a line to the user's shell configuration file
    /// (e.g., `.bashrc`, `.zshrc`) to export the PATH with the installation directory.
    ///
    /// On Windows, it runs a PowerShell command to update the PATH environment variable
    /// for the current user in a persistent way.
    pub fn prepend_install_dir_to_path(&self) -> Result<UpdateType, PathmanError> {
        prepend_to_path(&self.install_dir, Some("Biome installation dir"))
    }

    /// Makes the binary executable
    ///
    /// This function sets the executable permissions on the binary file.
    /// On Unix-like systems, it sets the permissions to `755`.
    /// On Windows, this is a no-op since .exe files are executable by default.
    fn make_executable(&self, bin: &PathBuf) -> Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let mut perms = std::fs::metadata(bin)
                .context("Failed to get metadata for the binary")?
                .permissions();

            perms.set_mode(0o755); // Set executable permissions

            std::fs::set_permissions(bin, perms)
                .context("Failed to set permissions for the binary")?;
        }
        Ok(())
    }
}
