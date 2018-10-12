use abscissa::{Callable, LoadConfig};
use std::path::PathBuf;

mod deploy;
mod help;
mod version;

pub use self::{deploy::DeployCommand, help::HelpCommand, version::VersionCommand};
use config::{CanisterConfig, CONFIG_FILE_NAME};

#[derive(Debug, Options)]
pub enum CanisterCommand {
    #[options(help = "deploy the application")]
    Deploy(DeployCommand),

    #[options(help = "show help for a command")]
    Help(HelpCommand),

    #[options(help = "display version information")]
    Version(VersionCommand),
}

impl_command!(CanisterCommand);

impl CanisterCommand {
    pub fn verbose(&self) -> bool {
        match self {
            CanisterCommand::Deploy(deploy) => deploy.verbose,
            _ => false,
        }
    }
}

impl LoadConfig<CanisterConfig> for CanisterCommand {
    fn config_path(&self) -> Option<PathBuf> {
        match self {
            CanisterCommand::Deploy(deploy) => Some(PathBuf::from(
                deploy
                    .config
                    .as_ref()
                    .map(|s| s.as_ref())
                    .unwrap_or(CONFIG_FILE_NAME),
            )),
            _ => None,
        }
    }
}

impl Callable for CanisterCommand {
    fn call(&self) {
        match self {
            CanisterCommand::Help(help) => help.call(),
            CanisterCommand::Deploy(deploy) => deploy.call(),
            CanisterCommand::Version(version) => version.call(),
        }
    }
}
