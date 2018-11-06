use abscissa::Callable;
use config::CanisterConfig;

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
        DeployCommand{config: self.config.clone(), verbose: self.verbose}.call();
    }
}