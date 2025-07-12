use std::{
    env::consts::{ARCH, OS},
    fmt::Display,
    path::PathBuf,
    process::Command,
};

use anyhow::{Context, Result};
use home::home_dir;

pub struct Platform {
    pub os: String,
    pub arch: String,
    pub libc: Libc,
    pub extension: String,
    pub shell: Option<Shell>,
}

pub enum Libc {
    Musl,
    Glibc,
}

impl Display for Libc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Libc::Musl => write!(f, "musl"),
            Libc::Glibc => write!(f, "glibc"),
        }
    }
}

pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Shell {
    pub fn config_file(&self) -> Result<PathBuf> {
        let home_dir = home_dir().context("Could not determine the home directory")?;

        let config_file = match self {
            Shell::Bash => match OS {
                "macos" => home_dir.join(".bash_profile"),
                _ => home_dir.join(".bashrc"),
            },
            Shell::Zsh => home_dir.join(".zshrc"),
            Shell::Fish => home_dir.join(".config/fish/config.fish"),
        };

        Ok(config_file)
    }
}

impl Platform {
    /// Detects the current platform
    pub fn detect() -> Self {
        Platform {
            os: OS.to_string(),
            arch: ARCH.to_string(),
            libc: Self::detect_libc(),
            extension: Self::detect_extension(),
            shell: Self::detect_shell(),
        }
    }

    /// Detects the libc implementation in use
    ///
    /// This function attempts to determine the libc implementation by checking
    /// the output of the `ldd --version` command on Linux systems. If the command
    /// fails or the OS is not Linux, it defaults to `Glibc`.
    fn detect_libc() -> Libc {
        match OS {
            "linux" => match Command::new("ldd").arg("--version").output() {
                Ok(output) => match String::from_utf8_lossy(&output.stdout) {
                    version if version.contains("musl") => Libc::Musl,
                    _ => Libc::Glibc,
                },
                Err(_) => Libc::Glibc,
            },
            _ => Libc::Glibc,
        }
    }

    /// Detects the file extension for the current platform
    fn detect_extension() -> String {
        match OS {
            "windows" => ".exe".to_string(),
            _ => "".to_string(),
        }
    }

    fn detect_shell() -> Option<Shell> {
        match std::env::var_os("SHELL") {
            Some(shell) if shell.to_string_lossy().contains("bash") => Some(Shell::Bash),
            Some(shell) if shell.to_string_lossy().contains("zsh") => Some(Shell::Zsh),
            Some(shell) if shell.to_string_lossy().contains("fish") => Some(Shell::Fish),
            _ => None,
        }
    }
}
