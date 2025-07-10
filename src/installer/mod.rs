use std::{
    env::var_os,
    fs::{create_dir_all, rename},
    path::PathBuf,
};

use anyhow::{Context, Result};

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
    pub fn prepend_install_dir_to_path(&self) -> Result<()> {
        #[cfg(unix)]
        {
            self.prepend_install_dir_to_path_unix()
                .context("Failed to prepend installation directory to PATH")
        }

        #[cfg(windows)]
        {
            self.prepend_install_dir_to_path_windows()
                .context("Failed to prepend installation directory to PATH")
        }
    }

    #[cfg(unix)]
    fn prepend_install_dir_to_path_unix(&self) -> Result<()> {
        let shell_config = &self.platform.shell.unwrap().config_file();


        if let Some(config_path) = shell_config {
            let mut config_content =
                std::fs::read_to_string(&config_path).unwrap_or_else(|_| String::new());

            println!("config path: {}", config_path.display());

            let export_line = format!(
                "\nexport PATH=\"{}:$PATH\"\n",
                self.install_dir.canonicalize().unwrap().to_string_lossy()
            );

            if !config_content.contains(&export_line) {
                config_content.push_str(&export_line);

                std::fs::write(&config_path, config_content)
                    .context("Failed to write to the shell configuration file")?;
            }

            return Ok(());
        }

        return Err(anyhow::anyhow!(
            "Could not determine the shell configuration file"
        ));
    }

    #[cfg(windows)]
    fn prepend_install_dir_to_path_windows(&self) -> Result<()> {
        use winreg::RegKey;
        use winreg::enums::*;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;
        let current_path = env.get_value("PATH").unwrap_or_default();

        let new_path = if current_path.is_empty() {
            self.install_dir.to_string_lossy()
        } else {
            format!("{};{}", self.install_dir.to_string_lossy(), current_path)
        };

        env.set_value("PATH", &new_path)
            .context("Failed to set PATH environment variable")?;

        Ok(())
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

    /// Checks if the installation directory is already in the PATH
    ///
    /// This function checks the current PATH environment variable
    /// and returns true if the installation directory is found.
    ///
    /// It does NOT check whether the export is present in the shell config file,
    /// because the user might have added it manually in a different way.
    pub fn is_install_dir_in_path(&self) -> Result<bool> {
        #[cfg(unix)]
        {
            if let Some(path_env) = var_os("PATH") {
                let current_path = path_env.to_string_lossy();
                return Ok(current_path
                    .split(':')
                    .any(|p| p.trim() == self.install_dir.to_string_lossy()));
            } else {
                Ok(false)
            }
        }

        #[cfg(windows)]
        {
            use winreg::RegKey;
            use winreg::enums::*;

            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let env = hkcu.open_subkey_with_flags("Environment", KEY_READ)?;
            let current_path: String = env.get_value("PATH").unwrap_or_default();

            Ok(current_path
                .split(';')
                .any(|p| p.trim() == self.install_dir.to_string_lossy()))
        }
    }
}
