use std::{path::PathBuf, process::Command};

use anyhow::{Context, Result};

use super::BiomeInstaller;

pub struct WindowsInstaller {
    install_dir: PathBuf,
}

impl BiomeInstaller for WindowsInstaller {
    /// Creates a new Biome installer for Linux
    fn new(install_dir: PathBuf) -> Self {
        WindowsInstaller { install_dir }
    }

    /// Returns the installation directory
    fn get_install_dir(&self) -> &PathBuf {
        &self.install_dir
    }

    /// Returns the name of the executable
    fn get_executable_name(&self) -> &str {
        "biome.exe"
    }

    /// Makes the binary executable
    /// On Windows, this is a no-op because .exe files are executable by default
    fn make_executable(&self, _bin: &PathBuf) -> Result<()> {
        Ok(())
    }

    /// Prepend the installation directory to the user's PATH
    fn update_path(&self) -> Result<()> {
        let script = format!(
            r#"
            $old = [Environment]::GetEnvironmentVariable('PATH', 'User');
            if (-not ($old.Split(';') -contains '{install_dir}')) {{ 
                [Environment]::SetEnvironmentVariable('PATH', $old + ';{install_dir}', 'User') 
            }}
            "#,
            install_dir = self.install_dir.display()
        );

        Command::new("powershell")
            .args(["-NoProfile", "-Command", &script])
            .output()
            .context("Failed to update PATH on Windows")?;

        Ok(())
    }
}
