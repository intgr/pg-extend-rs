// Copyright 2019 Marti Raudsepp <marti@juffo.org>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate pg_extern_attr;
extern crate pg_extend;

use pg_extern_attr::pg_extern;
use pg_extend::pg_magic;

// This tells Postgres this library is a Postgres extension
pg_magic!(version: pg_sys::PG_VERSION_NUM);


/// An error in PostgreSQL aborts the current statement and (sub)transaction.
#[pg_extern]
fn rs_error(msg: String) -> () {
    log::error!("{}", msg);
}

/// Log messages in all non-error log levels.
#[pg_extern]
fn rs_log_all() -> () {
    log::warn!("This is a warning");
    log::info!("This is an info message");
    log::debug!("This is a debug message");
    log::trace!("This is a trace message");
}

/// The NULLIF function returns a null value if value1 equals value2; otherwise it returns value1
/// https://www.postgresql.org/docs/current/functions-conditional.html#FUNCTIONS-NULLIF
#[pg_extern]
fn rs_nullif(value1: Option<String>, value2: Option<String>) -> Option<String> {
    if value1 == value2 { None } else { value1 }
}


#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_get_null() {
        assert_eq!(get_null(), None);
    }
    */
}
