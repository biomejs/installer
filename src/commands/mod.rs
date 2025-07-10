mod install;

use clap::Subcommand;
pub use install::InstallCommand;

#[derive(Subcommand)]

pub enum Commands {
    Install(InstallCommand),
}
