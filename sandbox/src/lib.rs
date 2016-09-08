#![feature(type_ascription)]
#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(serde_macros)]
#![feature(question_mark)]

extern crate rand;
extern crate time;
extern crate serde;
extern crate toml;
#[macro_use]
extern crate log;

extern crate clap;

extern crate exonum;
extern crate timestamping;
extern crate cryptocurrency;

mod sandbox;

mod tx_generator;
pub mod testnet;
mod config_file;

pub use tx_generator::TimestampingTxGenerator;
pub use config_file::ConfigFile;
pub use self::sandbox::timestamping_sandbox;
