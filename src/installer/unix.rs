use anyhow::{Context, Result};
use homedir::my_home;
use std::{fs, os::unix::fs::PermissionsExt};

use super::BiomeInstaller;
use std::{env, path::PathBuf};

pub struct UnixInstaller {
    install_dir: PathBuf,
}

impl BiomeInstaller for UnixInstaller {
    /// Creates a new Biome installer for Linux
    fn new(install_dir: PathBuf) -> Self {
        UnixInstaller { install_dir }
    }

    /// Returns the installation directory
    fn get_install_dir(&self) -> &PathBuf {
        &self.install_dir
    }

    /// Prepend the installation directory to the user's PATH
    fn update_path(&self) -> anyhow::Result<()> {
        match Shell::from_env() {
            Some(shell) => {
                if !shell.is_already_in_path(&self.install_dir) {
                    shell.append_to_path(&self.install_dir)?;
                    Ok(())
                } else {
                    println!(
                        "The installation directory {} is already in your PATH.",
                        self.install_dir.display()
                    );
                    Ok(())
                }
            }
            None => {
                println!(
                    "Could not detect shell. Please manually add {} to your PATH.",
                    self.install_dir.display()
                );
                Ok(())
            }
        }
    }

    /// Makes the given binary executable
    fn make_executable(&self, bin: &PathBuf) -> Result<()> {
        let mut permissions = fs::metadata(&bin)
            .context("Failed to get file metadata")?
            .permissions();

        permissions.set_mode(0o755);

        fs::set_permissions(&bin, permissions).context("Failed to set file permissions")?;

        Ok(())
    }
}

pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Shell {
    /// Detects the user's shell from the environment
    pub fn from_env() -> Option<Self> {
        let shell_path = env::var_os("SHELL");

        match shell_path {
            Some(path) if path.to_string_lossy().ends_with("bash") => Some(Shell::Bash),
            Some(path) if path.to_string_lossy().ends_with("zsh") => Some(Shell::Zsh),
            Some(path) if path.to_string_lossy().ends_with("fish") => Some(Shell::Fish),
            _ => None,
        }
    }

    /// Returns the shell-specific configuration file for updating PATH
    pub fn config_file(&self) -> Option<PathBuf> {
        let home_dir = my_home().ok()?;

        if let Some(home_dir) = home_dir {
            return match self {
                Shell::Bash => Some(home_dir.join(".bashrc")),
                Shell::Zsh => Some(home_dir.join(".zshrc")),
                Shell::Fish => Some(home_dir.join(".config/fish/config.fish")),
            };
        }

        None
    }

    /// Determines if the installation directory is already in the PATH
    pub fn is_already_in_path(&self, install_dir: &PathBuf) -> bool {
        let path = env::var_os("PATH");

        if let Some(path) = path {
            let install_dir = install_dir
                .canonicalize()
                .unwrap_or_else(|_| install_dir.clone());

            return path
                .to_string_lossy()
                .split(":")
                .any(|segment| PathBuf::from(segment) == install_dir);
        }

        return false;
    }

    /// Appends the installation directory to the shell's PATH in the configuration file
    ///
    /// This function works idempotently, meaning that it will only add the export line
    /// if it does not already exist in the configuration file.
    pub fn append_to_path(&self, install_dir: &PathBuf) -> Result<Option<PathBuf>> {
        if let Some(config_file) = self.config_file() {
            let mut file_content =
                std::fs::read_to_string(&config_file).unwrap_or_else(|_| String::new());

            let script = match self {
                Shell::Bash | Shell::Zsh => format!(
                    "export PATH=\"{}:$PATH\"\n",
                    install_dir.canonicalize()?.display()
                ),
                Shell::Fish => format!(
                    "set -gx PATH \"{}\" $PATH\n",
                    install_dir.canonicalize()?.display()
                ),
            };

            if !file_content.contains(&script) {
                file_content.push_str(&script);
                std::fs::write(&config_file, file_content)?;
            }

            return Ok(Some(config_file));
        }
        Ok(None)
    }
}
