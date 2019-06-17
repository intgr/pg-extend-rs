extern crate integration_tests;

use core::borrow::Borrow;
use core::mem;
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::panic::AssertUnwindSafe;
use std::sync::{Arc, Mutex};

use postgres::error::DbError;
use postgres::{Connection, HandleNotice};

use integration_tests::*;

#[test]
fn test_rs_error() {
    test_in_db("logging", |conn| {
        let result = conn
            .query("SELECT rs_error('No you dont!')", &[])
            .expect_err("error not thrown");
        assert_eq!(format!("{}", result), "database error: ERROR: No you dont!");
    });
}

/*
struct MsgCapture {
    msgs: Vec<DbError>
}

impl MsgCapture {
    fn drain(&mut self) -> Vec<DbError> {
        mem::replace(&mut self.msgs, Vec::new())
    }
}

impl HandleNotice for MsgCapture {
    fn handle_notice(&mut self, notice: DbError) {
        self.msgs.push(notice);
    }
}
*/

#[test]
fn test_rs_log_all() {
    test_in_db("logging", |conn: Connection| {
        let msgs__ = Arc::new(Mutex::new(Vec::new()));
        let msgs_ = msgs__.clone();

        let old_handler =
            conn.set_notice_handler(Box::new(move |msg| msgs_.lock().unwrap().push(msg)));

        // https://www.postgresql.org/docs/current/runtime-config-client.html#GUC-CLIENT-MIN-MESSAGES
        // TODO: document "ERROR" behavior
        // TODO: document that "INFO" messages are *always* sent to the client.
        //////
        conn.query("SET client_min_messages=error", &[])
            .expect("query failed");
        conn.query("SELECT rs_log_all()", &[])
            .expect("query failed");

        {
            let msgs = msgs__.lock().unwrap();
            assert_eq!(msgs[0].message, "This is an info message");
            assert_eq!(msgs.len(), 1);
        }
        msgs__.lock().unwrap().clear();

        //////
        conn.query("RESET client_min_messages", &[])
            .expect("query failed");
        conn.query("SELECT rs_log_all()", &[])
            .expect("query failed");

        {
            let msgs = msgs__.lock().unwrap();
            assert_eq!(msgs[0].message, "This is a warning");
            assert_eq!(msgs[1].message, "This is an info message");
            assert_eq!(msgs.len(), 2);
        }

        //////
        // Restore old notice handler
        conn.set_notice_handler(old_handler);
    });
}
