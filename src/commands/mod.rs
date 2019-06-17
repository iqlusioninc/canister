use abscissa_core::{Command, Configurable, Help, Options, Runnable};
use std::path::PathBuf;

mod backup;
mod deploy;
mod restore;
mod run;
mod version;

pub use self::{
    backup::BackupCommand, deploy::DeployCommand, restore::RestoreCommand,
    run::RunCommand, version::VersionCommand,
};
use crate::config::{CanisterConfig, CONFIG_FILE_NAME};

#[derive(Command, Debug, Options, Runnable)]
pub enum CanisterCommand {
    #[options(help = "backup application snapshot")]
    Backup(BackupCommand),

    #[options(help = "deploy application")]
    Deploy(DeployCommand),

    #[options(help = "show help for a command")]
    Help(Help<Self>),

    #[options(help = "restore application snapshot")]
    Restore(RestoreCommand),

    #[options(help = "run application")]
    Run(RunCommand),

    #[options(help = "display version information")]
    Version(VersionCommand),
}

impl CanisterCommand {
    pub fn verbose(&self) -> bool {
        match self {
            CanisterCommand::Backup(backup) => backup.verbose,
            CanisterCommand::Deploy(deploy) => deploy.verbose,
            CanisterCommand::Restore(restore) => restore.verbose,
            CanisterCommand::Run(run) => run.verbose,
            _ => false,
        }
    }
}

impl Configurable<CanisterConfig> for CanisterCommand {
    fn config_path(&self) -> Option<PathBuf> {
        match self {
            CanisterCommand::Backup(backup) => Some(PathBuf::from(
                backup
                    .config
                    .as_ref()
                    .map(|s| s.as_ref())
                    .unwrap_or(CONFIG_FILE_NAME),
            )),
            CanisterCommand::Deploy(deploy) => Some(PathBuf::from(
                deploy
                    .config
                    .as_ref()
                    .map(|s| s.as_ref())
                    .unwrap_or(CONFIG_FILE_NAME),
            )),
            CanisterCommand::Restore(restore) => Some(PathBuf::from(
                restore
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
