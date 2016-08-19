#![feature(type_ascription)]
#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(serde_macros)]
#![feature(question_mark)]

extern crate rand;
extern crate time;
extern crate exonum;
extern crate timestamping;
extern crate serde;
extern crate toml;
#[macro_use]
extern crate log;

extern crate clap;

#[cfg(test)]
mod sandbox;
#[cfg(test)]
mod tests;

mod tx_generator;
pub mod testnet;

pub use tx_generator::{TimestampingTxGenerator};
