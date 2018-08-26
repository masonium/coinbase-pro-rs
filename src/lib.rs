#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate pretty_env_logger;
extern crate serde;
extern crate time;
extern crate tokio;
extern crate uuid;

pub mod error;
mod private;
mod public;
pub mod structs;
mod utils;

use error::*;

pub type Result<T> = std::result::Result<T, CBError>;
