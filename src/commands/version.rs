//! The `version` subcommand
//!
#![allow(clippy::never_loop)]

use super::CanisterCommand;
use abscissa::Command as CommandTrait;

/// The `version` subcommand
#[derive(Debug, Default, Options)]
pub struct VersionCommand {}

impl VersionCommand {
    /// Print version message
    pub fn run(&self) {
        CanisterCommand::print_package_info();
    }
}
