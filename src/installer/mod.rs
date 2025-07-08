use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::PathBuf;

#[cfg(any(target_os = "linux", target_os = "macos"))]
mod unix;

#[cfg(target_os = "windows")]
mod windows;

pub struct Installer;

impl Installer {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn install(
        bin: PathBuf,
        install_dir: PathBuf,
        update_path: bool,
    ) -> Result<(PathBuf, PathBuf)> {
        unix::UnixInstaller::new(install_dir).install(bin, update_path)
    }

    #[cfg(target_os = "windows")]
    pub fn install(
        bin: PathBuf,
        install_dir: PathBuf,
        update_path: bool,
    ) -> Result<(PathBuf, PathBuf)> {
        windows::WindowsInstaller::new(install_dir).install(bin, update_path)
    }
}

trait BiomeInstaller {
    /// Creates a new Biome installer
    fn new(install_dir: PathBuf) -> Self;

    /// Installs the Biome CLI
    fn install(&self, bin: PathBuf, update_path: bool) -> Result<(PathBuf, PathBuf)> {
        let install_dir = self.get_install_dir();
        let destination_path = install_dir.join(self.get_executable_name());

        // Ensure installation directory exists
        if !install_dir.exists() {
            fs::create_dir_all(install_dir).context(anyhow!(
                "Could not create installation directory {}",
                install_dir.display()
            ))?;
        }

        fs::rename(&bin, &destination_path).context(anyhow!(
            "Could not copy binary {} to installation directory {}",
            bin.display(),
            install_dir.display()
        ))?;

        self.make_executable(&destination_path).context(anyhow!(
            "Could not make binary {} executable",
            destination_path.display()
        ))?;

        if update_path {
            self.update_path().context("Failed to update PATH")?;
        } else {
            println!("Skippping PATH update because `--no-update-path` was specified.");
        }

        Ok((destination_path, install_dir.clone()))
    }

    /// Returns the installation directory
    fn get_install_dir(&self) -> &PathBuf;

    /// Returns the name of the executable
    fn get_executable_name(&self) -> &str {
        "biome"
    }

    /// Makes the given binary executable
    fn make_executable(&self, bin: &PathBuf) -> Result<()>;

    /// Prepend the installation directory to the user's PATH
    fn update_path(&self) -> Result<()>;
}
