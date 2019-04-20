#[macro_use]
extern crate abscissa;
#[macro_use]
extern crate abscissa_derive;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod application;
mod commands;
mod config;
mod error;
mod gcp;
mod packer;
mod unpacker;

use crate::application::CanisterApplication;

fn main() {
    abscissa::boot(CanisterApplication);
}
