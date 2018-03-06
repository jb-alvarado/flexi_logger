#![warn(missing_docs)]

//! A logger that writes logs to `stderr` or to a fresh file,
//! or to a sequence of files in a configurable folder.
//! It allows custom logline formats, and it allows changing the log specification at runtime.
//! It further allows defining additional log streams, e.g. for alert or security messages.
//!
//! As it had started as an extended copy of [`env_logger`](http://crates.io/crates/env_logger/),
//! it is still using a similar syntax for specifying which logs should really be written.
//!
//! # Usage
//!
//! Add `flexi_logger` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! flexi_logger = "0.7"
//! log = "0.4"
//! ```
//!
//! and this to your crate root:
//!
//! ```text
//! extern crate flexi_logger;
//! #[macro_use]
//! extern crate log;
//! ```
//!
//! The latter is needed because `flexi_logger` plugs into the standard Rust logging facade given
//! by the [log crate](https://crates.io/crates/log),
//! and you use the ```log``` macros to write log lines from your code.
//!
//! Early in the start-up of your program, call something like
//!
//! ```text
//!    use flexi_logger::Logger;
//!
//!    Logger::with_env_or_str("modx::mody = info")
//!        // ... your configuration options go here ...
//!        .start()
//!        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
//! ```
//!
//! The configuration options allow e.g. to
//!
//! *  decide whether you want to write your default logs to stderr or to a file,
//! *  configure the filenames and the folders in which the log files are created,
//! *  specify the line format for the log lines
//! *  define additional log writers, e.g for special log types.
//!
//! See [Logger](struct.Logger.html) for a full description of all configuration options,
//! and the [writers](writers/index.html) module for the usage
//! of additional log writers.
//!

extern crate chrono;
extern crate glob;
extern crate log;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod primary_writer;
mod deprecated1;
mod flexi_error;
mod flexi_logger;
mod formats;
mod logger;
mod log_config;
mod log_specification;

pub mod writers;
/// Re-exports from log crate
pub use log::{Level, LevelFilter, Record};

/// Package for deprecated structs etc. These will be removed or made internal
/// in one of the next versions (let us know if this would cause difficulties).
pub mod deprecated {
    #[allow(deprecated)]
    pub use deprecated1::{init, LogOptions};
    pub use log_config::LogConfig;
    pub use flexi_logger::FlexiLogger;
}

pub use formats::*;
pub use log_specification::{LogSpecBuilder, LogSpecification};

pub use logger::Logger;
pub use flexi_logger::ReconfigurationHandle;
pub use flexi_error::FlexiLoggerError;
