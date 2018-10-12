#[macro_use]
extern crate abscissa;
#[macro_use]
extern crate abscissa_derive;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate hex;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate libflate;
extern crate percent_encoding;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;
extern crate tar;

mod application;
mod commands;
mod config;
mod error;
mod gcp;
mod unpacker;

use application::CanisterApplication;

fn main() {
    abscissa::boot(CanisterApplication);
}
