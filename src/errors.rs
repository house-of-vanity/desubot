use reqwest::Error as reqwest_error;
use rusqlite::Error as sqlite_error;
use serde_json::Error as serde_error;
use std::{fmt, io::Error as io_error};
use telegram_bot::Error as tg_error;

#[derive(Debug)]
pub(crate) enum Error {
    UserNotFound,
    SQLITE3Error(sqlite_error),
    TelegramError(tg_error),
    ReqwestError(reqwest_error),
    ConfNotFound,
    WordNotFound,
    WordInStopList,
    IOError(io_error),
    FileNotFound,
    JsonParseError(serde_error),
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

impl From<reqwest_error> for Error {
    fn from(e: reqwest_error) -> Error {
        return Error::ReqwestError(e);
    }
}

impl From<io_error> for Error {
    fn from(e: io_error) -> Error {
        return Error::IOError(e);
    }
}

impl From<serde_error> for Error {
    fn from(e: serde_error) -> Error {
        return Error::JsonParseError(e);
    }
}
