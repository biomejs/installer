use std::{
    env::consts::{ARCH, OS},
    io::Write,
    path::PathBuf,
};

use anyhow::{Context, Result, anyhow};
use semver::Version;

use crate::platform::{Libc, Platform};

/// Biome Downloader
pub struct Downloader {
    pub platform: Platform,
}

impl Downloader {
    /// Creates a new Downloader instance
    pub fn new() -> Self {
        Self {
            platform: Platform::detect(),
        }
    }

    /// Downloads the specified version of Biome
    ///
    /// This function downloads the specified version of Biome into a temporary
    /// file and returns the path to that file.
    pub fn download(&self, version: Version) -> Result<PathBuf> {
        let tag = self.get_git_tag(&version);

        let asset = self
            .get_asset_name()
            .context("Could not compute asset name")?;

        let url = format!("https://github.com/biomejs/biome/releases/download/{tag}/{asset}");

        let mut temp_file = tempfile::Builder::new()
            .tempfile()
            .context("Could not create temporary file")?;

        let response = reqwest::blocking::get(&url)?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to download Biome from {url}: {}",
                response.status()
            ));
        }

        let bytes = response.bytes().context("Failed to read response")?;

        temp_file
            .write_all(bytes.as_ref())
            .context("Failed to write to temporary file")?;

        let (_, path) = temp_file
            .keep()
            .context("Could not persist temporary file")?;

        Ok(path)
    }

    /// Computes the git tag for the specified version
    ///
    /// This function computes the git tag for the specified version of Biome.
    ///
    /// Versions prior to 2.0.0 use the `cli/vX.Y.Z` format, while versions
    /// 2.0.0 and later use the `@biomejs/biome@X.Y.Z` format
    fn get_git_tag(&self, version: &Version) -> String {
        match version.major {
            1 => format!("cli/v{version}"),
            _ => format!("@biomejs/biome@{version}"),
        }
    }

    /// Computes the name of the GitHub asset
    ///
    /// This function computes the name of the GitHub asset for the current
    /// operating system, architecture and libc implementation.
    ///
    /// The assets published by Biome use Node.js naming conventions with
    /// regard to the operating system, architecture, so we need to map
    /// the rust naming conventions to the Node.js naming conventions.
    fn get_asset_name(&self) -> Result<String> {
        let asset = match (
            self.platform.os.as_str(),
            self.platform.arch.as_str(),
            &self.platform.libc,
        ) {
            ("linux", "x86_64", Libc::Glibc) => "biome-linux-x64",
            ("linux", "x86_64", Libc::Musl) => "biome-linux-x64-musl",
            ("linux", "aarch64", Libc::Glibc) => "biome-linux-arm64",
            ("linux", "aarch64", Libc::Musl) => "biome-linux-arm64-musl",
            ("macos", "x86_64", _) => "biome-darwin-x64",
            ("macos", "aarch64", _) => "biome-darwin-arm64",
            ("windows", "x86_64", _) => "biome-win32-x64.exe",
            ("windows", "aarch64", _) => "biome-win32-arm64.exe",
            _ => Err(anyhow!(
                "Unsupported platform: {} {} {}",
                OS,
                ARCH,
                self.platform.libc
            ))?,
        };

        Ok(asset.to_string())
    }
}
