use abscissa::{Callable, LoadConfig};
use std::path::PathBuf;

mod backup;
mod deploy;
mod help;
mod run;
mod version;

pub use self::{
    backup::BackupCommand, deploy::DeployCommand, help::HelpCommand, run::RunCommand,
    version::VersionCommand,
};
use crate::config::{CanisterConfig, CONFIG_FILE_NAME};

#[derive(Debug, Options)]
pub enum CanisterCommand {
    #[options(help = "backup the application")]
    Backup(BackupCommand),

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
            CanisterCommand::Backup(backup) => backup.verbose,
            CanisterCommand::Deploy(deploy) => deploy.verbose,
            CanisterCommand::Run(run) => run.verbose,
            _ => false,
        }
    }
}

impl LoadConfig<CanisterConfig> for CanisterCommand {
    fn config_path(&self) -> Option<PathBuf> {
        match self {
            CanisterCommand::Backup(backup) => Some(PathBuf::from(
                backup
                    .config
                    .as_ref()
                    .map(AsRef::as_ref)
                    .unwrap_or(CONFIG_FILE_NAME),
            )),
            CanisterCommand::Deploy(deploy) => Some(PathBuf::from(
                deploy
                    .config
                    .as_ref()
                    .map(AsRef::as_ref)
                    .unwrap_or(CONFIG_FILE_NAME),
            )),
            CanisterCommand::Run(run) => Some(PathBuf::from(
                run.config
                    .as_ref()
                    .map(AsRef::as_ref)
                    .unwrap_or(CONFIG_FILE_NAME),
            )),
            _ => None,
        }
    }
}

impl Callable for CanisterCommand {
    fn call(&self) {
        match self {
            CanisterCommand::Backup(backup) => backup.call(),
            CanisterCommand::Deploy(deploy) => deploy.call(),
            CanisterCommand::Help(help) => help.call(),
            CanisterCommand::Run(run) => run.call(),
            CanisterCommand::Version(version) => version.call(),
        }
    }
}
