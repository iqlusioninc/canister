//! The `version` subcommand
//!
#![allow(clippy::never_loop)]

use super::CanisterCommand;
use abscissa_core::{Command, Options, Runnable};

/// The `version` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct VersionCommand {}

impl Runnable for VersionCommand {
    /// Print version message
    fn run(&self) {
        println!("{} {}", CanisterCommand::name(), CanisterCommand::version());
    }
}
