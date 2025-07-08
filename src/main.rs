use crate::installer::Installer;
use anyhow::{Context, Result};
use clap::{ArgAction, ValueHint, arg, command, value_parser};
use downloader::Downloader;
use homedir::my_home;
use semver::Version;
use std::path::PathBuf;
use utils::get_latest_stable_version;

mod downloader;
mod installer;
mod utils;

fn main() -> Result<()> {
    // Compute the default installation directory
    let default_install_dir = my_home()?
        .context("Failed to get home directory")?
        .join(".biome")
        .join("bin");

    let matches = command!()
        .arg(
            arg!([version] "The version of Biome to download")
                .env("BIOME_VERSION")
                .value_parser(value_parser!(Version))
                .value_hint(ValueHint::Other),
        )
        .arg(
            arg!(-d --"install-dir" <DIR> "The directory in which to install Biome")
                .env("BIOME_INSTALL_DIR")
                .default_value(default_install_dir.into_os_string())
                .value_hint(ValueHint::DirPath)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-n --"no-update-path" "Do not update the PATH environment variable")
                .action(ArgAction::SetTrue)
                .group("flags")
                .help_heading("Flags"),
        )
        .disable_version_flag(true)
        .get_matches();

    let version = if let Some(version) = matches.try_get_one::<Version>("version")? {
        version.clone()
    } else {
        println!("No version specified, using latest stable version.");
        // If no version is specified, get the latest stable version
        get_latest_stable_version()
            .context("Could not determine the latest stable version of Biome")?
    };

    let temp_file = Downloader::download(&version)
        .context("Could not download the specified version of Biome")?;

    let install_dir = matches
        .get_one::<PathBuf>("install-dir")
        .unwrap()
        .to_path_buf();

    let update_path = !matches.get_flag("no-update-path");

    let (bin, _) = Installer::install(temp_file, install_dir, update_path)?;

    println!("Biome installed successfully at: {}", bin.display());

    Ok(())
}
