use abscissa_core::{Command, Configurable, Help, Runnable};
use clap::Parser;
use std::path::PathBuf;

mod deploy;
mod run;
mod version;

pub use self::{deploy::DeployCommand, run::RunCommand, version::VersionCommand};
use crate::config::{CanisterConfig, CONFIG_FILE_NAME};

#[derive(Command, Debug, Default, Parser)]
pub enum CanisterCommand {
    #[options(help = "deploy application")]
    Deploy(DeployCommand),

    #[options(help = "show help for a command")]
    Help(Help<Self>),

    #[options(help = "run application")]
    Run(RunCommand),

    #[options(help = "display version information")]
    Version(VersionCommand),
}

/// Entry point for the application.
#[derive(Command, Debug, Clap)]
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
            _ => false,
        }
    }
}

impl Configurable<CanisterConfig> for EntryPoint {
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
