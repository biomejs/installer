use std::{io::Write, path::PathBuf};

use anyhow::{Context, Ok, Result};
use colored::Colorize;
use semver::Version;
use tempfile::NamedTempFile;

use crate::utils::is_musl;

/// Biome Downloader
pub struct Downloader;

impl Downloader {
    /// Downloads the specified version of Biome
    ///
    /// This function downloads the specified version of Biome for the current
    /// platform into a temporary file and returns the path to that file.
    pub fn download(version: &Version) -> Result<PathBuf> {
        // Retrieve the version-specific tag
        let tag = Self::get_tag(version);

        // Retrieve the platform-specific asset name
        let asset = Self::get_asset_name().context("Could not compute GitHub asset name")?;

        // Construct the GitHub URL for the downloading asset
        let url = format!("https://github.com/biomejs/biome/releases/download/{tag}/{asset}");

        // Create a temporary file to store the downloaded asset
        let temp_file = tempfile::Builder::new()
            .tempfile()
            .context("Could not create temporary file")?;

        println!(
            "{}",
            format!("Downloading Biome version {}", version).blue()
        );

        // Download the asset into the temporary file
        let temp_file_path =
            Self::download_to_file(url.as_str(), temp_file).context("Failed to download asset")?;

        Ok(temp_file_path)
    }

    /// Computes the git tag for the specified version of Biome
    ///
    /// Biome uses two different tag formats:
    /// - `v1.X.Y` versions use the `cli/v1.X.Y` tag format
    /// - Version starting from `2.0.0` use the `@biomejs/biome@2.X.Y` tag format
    ///
    /// This function returns the appropriate tag format based on major version
    fn get_tag(version: &Version) -> String {
        match version.major {
            1 => format!("cli/v{}", version),
            _ => format!("@biomejs/biome@{}", version),
        }
    }

    /// Computes the platform-specific asset name based on the os and
    /// architecture.
    ///
    /// Biome uses Node.js-style platform and architecture identifiers in asset
    /// names, such as:
    ///
    /// - `biome-linux-x64`
    /// - `biome-linux-arm64`
    /// - `biome-macos-x64`
    /// - `biome-macos-arm64`
    /// - `biome-win32-x64`
    /// - `biome-win32-arm64`
    ///
    /// This means we need to map Rust-style platform and architecture
    /// identifiers to Node.js-style identifiers.
    fn get_asset_name() -> Result<String> {
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        let musl = is_musl();

        let asset_name = match (os, arch, musl) {
            ("linux", "x86_64", false) => "biome-linux-x64",
            ("linux", "x86_64", true) => "biome-linux-x64-musl",
            ("linux", "aarch64", false) => "biome-linux-arm64",
            ("linux", "aarch64", true) => "biome-linux-arm64-musl",
            ("macos", "x86_64", false) => "biome-darwin-x64",
            ("macos", "aarch64", false) => "biome-darwin-arm64",
            ("windows", "x86_64", false) => "biome-win32-x64.exe",
            ("windows", "aarch64", false) => "biome-win32-arm64.exe",
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported platform and architecture combo: {}-{}",
                    os,
                    arch
                ));
            }
        };

        Ok(asset_name.to_string())
    }

    /// Downloads the file at the given URL into the specified temporary file.
    ///
    /// This function downloads the file at the specified URL and writes its
    /// contents to the provided temporary file. It then returns the path to
    /// the temporary file.
    fn download_to_file(url: &str, mut file: NamedTempFile) -> Result<PathBuf> {
        let response = reqwest::blocking::get(url)?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to download the file from {url}. Status: {}",
                response.status()
            ));
        }

        let content = response.bytes()?;

        file.write_all(content.as_ref())
            .context("Failed to write to temporary file")?;

        let (_, path) = file.keep()?;

        Ok(path)
    }
}
