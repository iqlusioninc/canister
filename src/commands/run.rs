use abscissa::{Callable, GlobalConfig};
use config::CanisterConfig;
use std::process::Command;

use super::DeployCommand;

#[derive(Debug, Options)]
pub struct RunCommand {
    #[options(short = "c", long = "config")]
    pub config: Option<String>,

    #[options(short = "v", long = "verbose")]
    pub verbose: bool,
}

impl Default for RunCommand {
    fn default() -> Self {
        Self {
            config: None,
            verbose: false,
        }
    }
}

impl Callable for RunCommand {
    fn call(&self) {
        let config = CanisterConfig::get_global();
        let path = &config.run_command.path;
        let args = &config.run_command.args;

        DeployCommand {
            config: self.config.clone(),
            verbose: self.verbose,
        }
        .call();

        let mut run_command = Command::new(path.clone()).args(args).spawn().unwrap();
        run_command.wait().unwrap();
    }
}
