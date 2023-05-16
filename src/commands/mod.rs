use abscissa_core::{Command, Configurable, Runnable};
use clap::Parser;
use std::path::PathBuf;

mod deploy;
mod run;
mod version;

pub use self::{deploy::DeployCommand, run::RunCommand, version::VersionCommand};
use crate::config::{CanisterConfig, CONFIG_FILE_NAME};

#[derive(Command, Debug, Parser, Runnable)]
pub enum CanisterCommand {
    /// subcommands for Deploy
    Deploy(DeployCommand),

    /// subcommands for Run
    Run(RunCommand),
}

impl CanisterCommand {
    pub fn verbose(&self) -> bool {
        match self {
            CanisterCommand::Deploy(deploy) => deploy.verbose,
            CanisterCommand::Run(run) => run.verbose,
        }
    }
}

impl Configurable<CanisterConfig> for CanisterCommand {
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
        }
    }
}
