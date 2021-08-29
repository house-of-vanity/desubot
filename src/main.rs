#![allow(unreachable_code)]
use std::{env, process};
use tokio::time::{delay_for, Duration};

use futures::StreamExt;
use telegram_bot::*;
#[macro_use]
extern crate log;
use env_logger::Env;

mod commands;
mod db;
mod errors;
mod handlers;
mod utils;

use mystem::MyStem;

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let mut mystem = match MyStem::new() {
        Ok(mystem) => mystem,
        Err(e) => {
            error!("MyStem init error. {:?}", e);
            process::exit(0x0002);
        }
    };
    match db::update_scheme() {
        Ok(_) => {}
        Err(e) => panic!("Database error: {:?}", e),
    }
    match db::load_stopwords() {
        Ok(_) => {}
        Err(e) => panic!("Database error: {:?}", e),
    }
    let token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            error!("TELEGRAM_BOT_TOKEN not set");
            process::exit(0x0001);
        }
    };
    let api = Api::new(token.clone());
    let mut stream = api.stream();
    let me = api.send(GetMe).await?;
    info!(
        "GetMe result: Username: {}, First Name: {}, ID {}",
        me.username.as_ref().unwrap(),
        me.first_name,
        me.id
    );
    loop {
        while let Some(update) = stream.next().await {
            match update {
                Ok(u) => {
                    if let UpdateKind::Message(message) = u.kind {
                        db::add_conf(message.clone()).await?;
                        db::add_user(message.clone()).await?;
                        match handlers::handler(
                            api.clone(),
                            message,
                            token.clone(),
                            &mut mystem,
                            me.clone(),
                        )
                        .await
                        {
                            Ok(_) => {}
                            Err(e) => warn!("An error occurred handling command. {:?}", e),
                        }
                    }
                }
                Err(e) => {
                    warn!("Telegram API Error: {:?}", e);
                }
            };
        }
        delay_for(Duration::from_secs(2)).await;
    }
    Ok(())
}
