extern crate irc;

use std::borrow::Borrow;

use futures::prelude::*;
use irc::client::prelude::*;

use crate::actions::{Executable, ListDir};

mod actions;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {

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
                if message_txt.contains("LSDIRS") {
                    let list_dir = ListDir { arguments: "/home/asocha".to_string(), msg: message.to_owned() };
                    let result = list_dir.execute();
                    print!("{}", &*result);
                    client.send_privmsg(&channel, "ACK: ".to_owned() + result.as_ref()).unwrap();
                }
            }
            _ => (),
        }
    }

    Ok(())
}
