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

/// Entry point for the application.
#[derive(Command, Debug, Parser)]
#[clap(author, about, version)]
pub struct EntryPoint {
    /// CanisterCommand
    #[clap(subcommand)]
    cmd: CanisterCommand,

    /// Enable verbose logging
    #[clap(short, long)]
    pub verbose: bool,

    /// Use the specified config file
    #[clap(short, long)]
    pub config: Option<String>,
}

impl Runnable for EntryPoint {
    fn run(&self) {
        self.cmd.run()
    }
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
