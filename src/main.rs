//! Canister

pub mod application;
pub mod commands;
pub mod config;
pub mod error;
pub mod gcp;
pub mod https_client;
pub mod packer;
pub mod prelude;
pub mod unpacker;

use crate::application::APPLICATION;

fn main() {
    abscissa_core::boot(&APPLICATION);
}
