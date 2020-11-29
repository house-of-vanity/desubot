use rusqlite::Error as sqlite_error;
use rusqlite::{named_params, params, Connection, Result};
use telegram_bot::Error as tg_error;
use std::{fmt, io};

#[derive(Debug)]
pub(crate) enum Error {
    UserNotFound,
    SQLITE3Error(sqlite_error),
    TelegramError(tg_error),
    ConfNotFound,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred.")
    }
}

impl From<sqlite_error> for Error {
    fn from(e: sqlite_error) -> Error {
        return Error::SQLITE3Error(e);
    }
}

impl From<tg_error> for Error {
    fn from(e: tg_error) -> Error {
        return Error::TelegramError(e);
    }
}