use abscissa::{Application, LoggingConfig};

use commands::CanisterCommand;
use config::CanisterConfig;

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
