use abscissa::Command;

use super::CanisterCommand;

/// The `help` subcommand
#[derive(Debug, Default, Options)]
pub struct HelpCommand {
    #[options(free)]
    pub args: Vec<String>,
}
impl HelpCommand {
    /// Print help message
    pub fn call(&self) -> ! {
        CanisterCommand::print_usage(self.args.as_slice())
    }
}
