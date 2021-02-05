extern crate base64;
extern crate irc;

use std::error::Error;

use chrono::{DateTime, Local, SecondsFormat, TimeZone};
use futures::prelude::*;
use irc::client::prelude::*;
use log::{debug, error, info, trace, warn};

use crate::actions::{Executable, ListDir};

mod actions;

const LS_DIR_CMD: &'static str = "ls dir";
const VPN_START_CMD: &'static str = "vpn start";
const VPN_START_TOKEN: &'static str = "token";


fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S%.3f]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    setup_logger();

    // We can also load the Config at runtime via Config::load("path/to/config.toml")
    let config = Config {
        nickname: Some("asochatst".to_owned()),
        server: Some("chat.freenode.net".to_owned()),
        channels: vec!["#asochatst".to_owned()],
        ..Config::default()
    };
    let mut client = Client::from_config(config).await?;
    client.identify()?;


    let mut stream = client.stream()?;
    while let Some(message) = stream.next().await.transpose()? {
        match message.command {
            Command::PRIVMSG(ref channel, ref message_txt) => {
                let msg = message_txt.trim_start().to_lowercase();

                //TODO implement pattern matching using my custom commands
                if msg.starts_with(LS_DIR_CMD) {
                    let msg_tokens: Vec<&str> = msg.split(LS_DIR_CMD).collect();
                    let ls_path = msg_tokens[1].trim();
                    if !ls_path.is_empty() {
                        debug!("DEBUG {} {}", msg_tokens[0], msg_tokens[1]);
                        let list_dir = ListDir { arguments: ls_path.to_string(), msg: message.to_owned() };
                        let results = &list_dir.execute().replace("\n", ", ");
                        debug!("DEBUG ls dir: {}", results);
                        let response = format!("{}", results);
                        client.send_privmsg(&channel, response).unwrap();
                    } else {
                        debug!("DEBUG ls path not provided");
                    }
                    //TODO implement pattern matching using my custom commands
                } else if msg.starts_with(VPN_START_CMD) {
                    if msg.contains(VPN_START_TOKEN) {
                        let msg_tokens: Vec<&str> = message_txt.split(VPN_START_TOKEN).collect();
                        match base64::decode(msg_tokens[1].trim()) {
                            Ok(vpn_token) => {
                                //TODO encrypt & decrypt
                                client.send_privmsg(&channel, vpn_token).unwrap();
                            }
                            Err(err) => {
                                client.send_privmsg(&channel, format!("Could not decode vpn token: {}", err)).unwrap();
                            }
                        };
                    } else {
                        client.send_privmsg(&channel, "Incomplete command, not token found".to_owned()).unwrap();
                    }
                } else {
                    client.send_privmsg(&channel, "Unrecognized command: ".to_owned() + message_txt.as_ref()).unwrap();
                }
            }
            _ => (),
        }
    }

    Ok(())
}
