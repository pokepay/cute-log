//!This crate provides simple and cute logger.
//!
//!## Feautres
//!
//!- `timestamp` - Enables timestamps in logs by means of `chrono`. Enabled by default
//!
//!## Usage
//!
//!```rust
//!#[macro_use]
//!extern crate log;
//!extern crate cute_log;
//!
//!fn main() {
//!    cute_log::init();
//!    info!("it works!");
//!}
//!```

extern crate log;
#[cfg(feature="timestamp")]
extern crate chrono;

use std::fmt;

///Simple Logger implementation
///
///It provides logger without filtering with following format:
///`<level> [<date and time>] <file>:<line> - <args>`
///
///Timestamp can be turned off by disabling default features
pub struct Logger;

impl Logger {
    #[cfg(feature="timestamp")]
    fn get_date() -> impl fmt::Display {
        chrono::offset::Local::now().format("%F %H:%M:%S%.3f %z")
    }

    ///Logger printer.
    pub fn print(record: &log::Record) {
        #[cfg(feature="timestamp")]
        {
            println!("{:<5} [{}] {}:{} - {}",
                     record.level(),
                     Self::get_date(),
                     record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0),
                     record.args());

        }

        #[cfg(not(feature="timestamp"))]
        {
            println!("{:<5} {}:{} - {}",
                     record.level(),
                     record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0),
                     record.args());
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        Self::print(record);
    }

    fn flush(&self) {
    }
}

///Sets global logger
pub fn init() -> Result<(), log::SetLoggerError> {
    static INSTANCE: Logger = Logger;
    log::set_logger(&INSTANCE)
}