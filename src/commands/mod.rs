use abscissa::{Callable, LoadConfig};
use std::path::PathBuf;

mod deploy;
mod help;
mod run;
mod version;

pub use self::{
    deploy::DeployCommand, help::HelpCommand, run::RunCommand, version::VersionCommand,
};
use config::{CanisterConfig, CONFIG_FILE_NAME};

#[derive(Debug, Options)]
pub enum CanisterCommand {
    #[options(help = "deploy the application")]
    Deploy(DeployCommand),

    #[options(help = "show help for a command")]
    Help(HelpCommand),

    #[options(help = "run the application")]
    Run(RunCommand),

    #[options(help = "display version information")]
    Version(VersionCommand),
}

impl_command!(CanisterCommand);

impl CanisterCommand {
    pub fn verbose(&self) -> bool {
        match self {
            CanisterCommand::Deploy(deploy) => deploy.verbose,
            CanisterCommand::Run(run) => run.verbose,
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
            CanisterCommand::Run(run) => Some(PathBuf::from(
                run.config
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
            CanisterCommand::Deploy(deploy) => deploy.call(),
            CanisterCommand::Help(help) => help.call(),
            CanisterCommand::Run(run) => run.call(),
            CanisterCommand::Version(version) => version.call(),
        }
    }
}
