use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{ArgAction, Args, ValueHint, arg, value_parser};
use colored::Colorize;
use home::home_dir;
use inquire::{Confirm, Select};
use pathman::UpdateType;
use semver::Version;
use spinners::{Spinner, Spinners};

use crate::{downloader::Downloader, installer::Installer, platform::Platform};

#[derive(Args, Clone, Debug)]
pub struct InstallCommand {
    /// The version of Biome to download and install
    #[arg(
        short,
        long,
        value_name = "VERSION",
        env = "BIOME_VERSION",
        value_hint = ValueHint::Other,
        value_parser = value_parser!(Version),
        help = "The version of Biome to download and install.",
    )]
    version: Option<Version>,

    #[arg(
        short,
        long,
        value_name = "DIR",
        env = "BIOME_INSTALL_DIR",
        value_hint = ValueHint::DirPath,
        value_parser = value_parser!(PathBuf),
        help = "The directory in which to install Biome",
    )]
    install_dir: Option<PathBuf>,

    /// Do prepend the installation directory to the PATH environment variable
    #[arg(
        short,
        long,
        env = "BIOME_PREPEND_PATH",
        action = ArgAction::SetTrue,
        help = "Do not prepend the installation directory to the PATH environment variable",
        help_heading = "Flags",
        group = "flags",
    )]
    no_prepend_path: bool,

    /// Run the installer in non-interactive mode
    #[arg(
        short = 'N',
        long,
        env = "BIOME_NON_INTERACTIVE",
        action = ArgAction::SetTrue,
        help = "Run the installer in non-interactive mode",
        help_heading = "Flags",
        group = "flags",
    )]
    non_interactive: bool,
}

impl InstallCommand {
    pub fn handle(&self) -> Result<()> {
        let version = match self.should_prompt() {
            true => self.prompt_version()?,
            false => self.version.clone().unwrap_or(self.get_latest_version()?),
        };

        let temp_file = self
            .download(version.clone())
            .context("Failed to download the specified version of Biome")?;

        let install_dir = match &self.install_dir {
            Some(dir) => dir.to_owned(),
            None => home_dir()
                .context("Could not determine the home directory")?
                .join(".biome")
                .join("bin"),
        };

        let installer = Installer::new(install_dir.clone());

        let destination = installer
            .install(temp_file)
            .context("Failed to install Biome")?;

        println!(
            "{}",
            format!(
                "✔ Biome has been installed to {}",
                format!("{}", destination.display()).bold()
            )
            .green()
        );

        self.prepend_install_dir_to_path_if_needed(&installer, install_dir)?;

        println!(
            "\n{}\n{}\n{}",
            format!(
                "❤️ Thank you for installing {}!",
                format!("Biome").blue().bold()
            ),
            format!(
                "⭐ Support the project — star us on GitHub! {}",
                format!("https://github.com/biomejs/biome").underline()
            ),
            format!(
                "📖 Learn more about Biome at {}",
                format!("https://biomejs.dev").underline()
            )
        );

        Ok(())
    }

    /// Prompts the user to choose a version of Biome to install
    ///
    /// This function will display a list of available versions and allow the
    /// user to select one.
    fn prompt_version(&self) -> Result<Version> {
        let (latest, versions) = self
            .get_versions()
            .context("Could not retrieve the list of versions")?;

        match Select::new("Please choose a version:", versions).prompt() {
            Ok(version) => Ok(version),
            Err(_) => Ok(latest),
        }
    }

    /// Prompts the user to ask them if they want to update their PATH
    fn prompt_update_path(&self) -> Result<bool> {
        #[cfg(unix)]
        {
            let platform = Platform::detect();

            let shell_config: Option<PathBuf> = match platform.shell {
                Some(shell) => match shell.config_file() {
                    Ok(path) => Some(path),
                    Err(_) => None,
                },
                None => None,
            };

            if let Some(shell_config) = shell_config {
                return Ok(Confirm::new("Do you want to update your PATH?")
                    .with_default(true)
                    .with_help_message(&format!(
                        r#"We'll add the export to your shell config file at {}"#,
                        shell_config.display(),
                    ))
                    .prompt()?);
            }

            return Ok(false);
        }

        #[cfg(windows)]
        {
            return Ok(Confirm::new("Do you want to update your PATH?")
                .with_default(true)
                .with_help_message(
                    "We will prepend the installation directory to your PATH environment variable",
                )
                .prompt()?);
        }
    }

    /// Fetches the list of available versions of Biome
    ///
    /// This function retrieves the list of available Biome versions from our
    /// version API and returns the latest version along with all available
    /// versions.
    fn get_latest_version(&self) -> Result<Version> {
        let mut spinner = Spinner::new(Spinners::Dots, "Fetching latest version...".into());

        let response = reqwest::blocking::get("https://biomejs.dev/api/versions/latest.txt")
            .context("Failed to fetch the latest version of Biome")?;

        let version = response
            .text()
            .context("Failed to read response")?
            .trim()
            .to_string()
            .parse::<Version>()
            .context("Failed to parse latest version")?;

        spinner.stop_and_persist(
            &"✔".green().to_string(),
            format!("Latest version is: {}", format!("{version}").bold())
                .green()
                .to_string(),
        );

        Ok(version)
    }

    /// Fetches the list of available versions of Biome
    ///
    /// This function retrieves the list of available Biome versions from our
    /// version API and returns the latest version along with all available
    /// versions.
    fn get_versions(&self) -> Result<(Version, Vec<Version>)> {
        let mut spinner = Spinner::new(Spinners::Dots, "Fetching the list of versions...".into());

        let response = reqwest::blocking::get("https://biomejs.dev/api/versions/stable.txt")
            .context("Failed to fetch the list of Biome version")?;

        let versions: Vec<Version> = response
            .text()
            .context("Failed to read response")?
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .filter_map(|line| Version::parse(&line).ok())
            .collect();

        let latest = versions.first().context("No versions available")?.clone();

        spinner.stop_and_persist(
            &"✔".green().to_string(),
            format!("Fetching the list of versions"),
        );

        Ok((latest, versions))
    }

    /// Checks if the installer should prompt the user for input
    ///
    /// This function determines whether the installer should prompt the user
    /// for inputs based on the `non_interactive` flag and whether the
    /// terminal supports interactivity, or is running in a CI environment.
    ///
    /// We first check the flag set by the user, then check if running in a CI
    /// environment because some CI envs simulate an interactive terminal,
    /// and finally check if the terminal supports interactivity.
    fn should_prompt(&self) -> bool {
        let is_interactive = atty::is(atty::Stream::Stdin) && atty::is(atty::Stream::Stdout);

        // If the CI environment variable is set, we assume we're running
        // in a CI environment.
        let runs_in_ci = std::env::var_os("CI").is_some();

        return !self.non_interactive && !runs_in_ci && is_interactive;
    }

    /// Downloads the specified version of Biome
    ///
    /// This function downloads the specified version of Biome and returns the
    /// path to the downloaded file.
    fn download(&self, version: Version) -> Result<PathBuf> {
        let mut spinner = Spinner::new(
            Spinners::Dots,
            format!("Downloading Biome {}", &version).into(),
        );

        let downloader = Downloader::new();

        let temp_file = downloader.download(version.clone())?;

        spinner.stop_and_persist(
            &"✔".green().to_string(),
            format!("Downloaded Biome {}", &version).green().to_string(),
        );

        Ok(temp_file)
    }

    fn prepend_install_dir_to_path_if_needed(
        &self,
        installer: &Installer,
        install_dir: PathBuf,
    ) -> Result<()> {
        // If the user has explicitly told us not to prepend the
        // installation directory to the PATH environment variable, we're done
        if self.no_prepend_path {
            println!(
                "{}",
                format!("As requested, the installation directory will not be added to your PATH.")
                    .yellow()
            );
            return Ok(());
        }

        // Otherwise, ask the user if possible, or just update the PATH in
        // non-interactive environments
        let should_update_path = match self.should_prompt() {
            true => self.prompt_update_path(),
            false => Ok(true),
        };

        // We're done if the PATH update was not requested, but we'll tell the
        // user to update their PATH manually
        if !should_update_path? {
            println!(
                "{}",
                format!(
                    "⚠ Please update your PATH manually to include: {}",
                    install_dir.display()
                )
                .yellow()
            );

            return Ok(());
        }

        match installer.prepend_install_dir_to_path() {
            Ok(update_type) => match update_type {
                UpdateType::Success => {
                    println!(
                        "{}",
                        format!(
                            "✔ The installation directory {} has been added to your PATH",
                            install_dir.display()
                        )
                        .green()
                    );

                    println!(
                        "{}",
                        format!("You may need to restart your terminal to apply the changes.")
                            .green()
                    );
                }
                UpdateType::AlreadyInPath => {
                    println!(
                        "{}",
                        format!(
                            "The installation directory {} is already in your PATH",
                            install_dir.display()
                        )
                        .yellow()
                    );
                }
            },
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to update the PATH environment variable: {}",
                    e
                ));
            }
        };

        Ok(())
    }
}
