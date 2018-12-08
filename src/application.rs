use abscissa::{Application, LoggingConfig};

use crate::commands::CanisterCommand;
use crate::config::CanisterConfig;

#[derive(Debug)]
pub struct CanisterApplication;

impl Application for CanisterApplication {
    type Cmd = CanisterCommand;
    type Config = CanisterConfig;

    fn logging_config(&self, command: &CanisterCommand) -> LoggingConfig {
        if command.verbose() {
            LoggingConfig::verbose()
        } else {
            LoggingConfig::default()
        }
    }
}
