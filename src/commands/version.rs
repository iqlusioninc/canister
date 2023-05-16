//! The `version` subcommand
//!
#![allow(clippy::never_loop)]

use abscissa_core::{Command, Runnable};
use clap::Parser;
use std::{option_env, process};

/// The `version` subcommand
#[derive(Command, Debug, Default, Parser)]
pub struct VersionCommand {}

impl Runnable for VersionCommand {
    /// Print version message
    fn run(&self) {
        println!("{}", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"));
        process::exit(0);
    }
}
