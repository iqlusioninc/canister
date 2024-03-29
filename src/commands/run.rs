use crate::prelude::*;
use abscissa_core::{Command, Runnable};
use clap::Parser;

use super::DeployCommand;

#[derive(Command, Debug, Default, Parser)]
pub struct RunCommand {
    #[clap(short = 'c', long = "config")]
    pub config: Option<String>,

    #[clap(short = 'v', long = "verbose")]
    pub verbose: bool,
}

impl Runnable for RunCommand {
    fn run(&self) {
        let config = APPLICATION.config();
        let path = &config.run_command.path;
        let args = &config.run_command.args;

        DeployCommand {
            config: self.config.clone(),
            verbose: self.verbose,
        }
        .run();

        let mut run_command = std::process::Command::new(path.clone())
            .args(args)
            .spawn()
            .unwrap();
        match run_command.wait() {
            Ok(exit_status) => match exit_status.code() {
                Some(0) => info!("successful exit status! cmd: {:?}", path),
                Some(code) => error!("error exit status! cmd: {:?}, code: {}", path, code),
                None => error!("Process terminated by unknown signal! cmd: {:?}", path),
            },
            Err(e) => error!("error: {}, cmd: {:?}", e, path),
        }
    }
}
