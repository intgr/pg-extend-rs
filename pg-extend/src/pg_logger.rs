// Copyright 2018 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Implements a Rust logger for PostgreSQL logging system

use crate::pg_error;
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::sync::atomic::{AtomicBool, Ordering};

struct PostgresLogger {}

static LOGGER: PostgresLogger = PostgresLogger {};
static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initialize the Rust logger middleware for PostgreSQL
pub fn init() {
    let initialized = INITIALIZED.fetch_or(false, Ordering::SeqCst);
    if !initialized {
        if let Err(err) = log::set_logger(&LOGGER) {
            // Use whatever logger is initialized to issue a warning
            log::warn!("Cannot set pg_logger: {}", err);
        } else {
            log::set_max_level(LevelFilter::Trace);
        }

        log::trace!("pg-extend initialized");
    }
}

impl Log for PostgresLogger {
    // TODO: Can we figure out from Postgres if this message will be ignored?
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let pg_level = match record.level() {
            Level::Error => pg_error::Level::Error,
            Level::Warn => pg_error::Level::Warning,
            Level::Info => pg_error::Level::Info,
            Level::Debug => pg_error::Level::Debug1,
            Level::Trace => pg_error::Level::Debug5,
        };
        pg_error::log(
            pg_level,
            record.file().unwrap_or("???"),
            record.line().unwrap_or(0),
            record.module_path().unwrap_or(""),
            format!("{}", record.args()),
        );
    }

    fn flush(&self) {}
}
