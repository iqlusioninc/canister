use abscissa::{Command, Runnable};

use super::CanisterCommand;

/// The `help` subcommand
#[derive(Debug, Default, Options)]
pub struct HelpCommand {
    #[options(free)]
    pub args: Vec<String>,
}
impl Runnable for HelpCommand {
    /// Print help message
    fn run(&self) {
        CanisterCommand::print_usage(
            &self
                .args
                .as_slice()
                .iter()
                .map(|arg| arg.as_ref())
                .collect::<Vec<_>>(),
        );
    }
}
