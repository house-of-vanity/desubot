use mystem::AppError as mystem_error;
use reqwest::Error as reqwest_error;
use rusqlite::Error as sqlite_error;
use serde_json::Error as serde_error;
use std::{fmt, io::Error as io_error};
use subprocess::PopenError as popen_error;
use telegram_bot::Error as tg_error;

#[derive(Debug)]
pub enum Error {
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
    PopenError(popen_error),
    MystemError(mystem_error),
    SQLBannedCommand(String),
    SQLInvalidCommand,
    SQLResultTooLong(String),
    CodeHighlightningError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred.")
        //         match self {
        //             _ => write!(f, "An error occurred."),
        //             // Error::UserNotFound => {}
        //             // Error::SQLITE3Error(_) => {}
        //             // Error::TelegramError(_) => {}
        //             // Error::ReqwestError(_) => {}
        //             // Error::ConfNotFound => {}
        //             // Error::WordNotFound => {}
        //             // Error::WordInStopList => {}
        //             // Error::IOError(_) => {}
        //             // Error::FileNotFound => {}
        //             // Error::JsonParseError(_) => {}
        //             // Error::PopenError(_) => {}
        //             // Error::MystemError(_) => {}
        //             // Error::SQLBannedCommand(_) => {}
        //             // Error::SQLInvalidCommand => {}
        //             // Error::SQLResultTooLong(_) => {}
        // //             Error::CodeHighlightningError(Help) => write!(f, "Code highlighter.\
        // // <b>Usage</b><pre>/CODE\
        // // #&lt;theme&gt;\
        // // &lt;CODE&gt;\
        // // #&lt;lang&gt;</pre>\
        // // \
        // // List of themes:\
        // // .")
        //             Error::CodeHighlightningError(help) => write!(f, "{}", help.description)
        //         }
    }
}

impl From<sqlite_error> for Error {
    fn from(e: sqlite_error) -> Error {
        Error::SQLITE3Error(e)
    }
}

impl From<tg_error> for Error {
    fn from(e: tg_error) -> Error {
        Error::TelegramError(e)
    }
}

impl From<reqwest_error> for Error {
    fn from(e: reqwest_error) -> Error {
        Error::ReqwestError(e)
    }
}

impl From<io_error> for Error {
    fn from(e: io_error) -> Error {
        Error::IOError(e)
    }
}

impl From<serde_error> for Error {
    fn from(e: serde_error) -> Error {
        Error::JsonParseError(e)
    }
}

impl From<popen_error> for Error {
    fn from(e: popen_error) -> Error {
        Error::PopenError(e)
    }
}

impl From<mystem_error> for Error {
    fn from(e: mystem_error) -> Error {
        Error::MystemError(e)
    }
}
