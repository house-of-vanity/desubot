use crate::errors;
use serde_json::Value;
use std::io::{Error, Write, BufReader, prelude::*};
use subprocess::{Popen, PopenConfig, PopenError, Redirection};

pub struct MyStem {
    pub process: Popen,
}

impl MyStem {
    pub fn new() -> Result<Self, PopenError> {
        Ok(Self {
            process: MyStem::open_process()?,
        })
    }

    fn open_process() -> Result<Popen, PopenError> {
        Popen::create(
            &["mystem", "-d", "--format", "json"],
            PopenConfig {
                stdout: Redirection::Pipe,
                stdin: Redirection::Pipe,
                ..Default::default()
            },
        )
    }

    #[allow(dead_code)]
    pub fn terminate(&mut self) -> Result<(), Error> {
        self.process.terminate()
    }

    #[allow(unused_must_use)]
    pub async fn stemming(&mut self, text: String) -> Result<Vec<String>, errors::Error> {
        if let Some(exit_status) = self.process.poll() {
            warn!(
                "MyStem process exited with: {:?}. Restarting...",
                exit_status
            );
            self.process = MyStem::open_process()?;
        }
        let mut words: Vec<String> = vec![];
        let clean_text = format!("{}{}", text.trim(), "\n");
        self.process
            .stdin
            .as_ref()
            .unwrap()
            .write(clean_text.as_bytes());
        let mut contents = String::new();
        let mut buf_reader = BufReader::new(self.process.stdout.as_ref().unwrap());
        buf_reader.read_line(&mut contents);

        match Some(contents) {
            Some(contents) => {
                let v: Vec<Value> = match serde_json::from_str(contents.as_str()) {
                    Ok(val) => val,
                    Err(_) => return Ok(vec![]),
                };
                for i in v {
                    words.push(i["analysis"][0]["lex"].to_string().replace("\"", ""));
                }
                words.retain(|x| x != "null");
                debug!(
                    "Mystem PID: {}. Parsed words: {}.",
                    self.process.pid().unwrap(),
                    words.join(", ")
                );
                Ok(words)
            }
            None => return Ok(vec![]),
        }
    }
}
