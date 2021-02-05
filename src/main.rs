extern crate base64;
extern crate irc;

use futures::prelude::*;
use irc::client::prelude::*;

use lsdir::ListDir;

use crate::actions::Executable;
use crate::vpn::ConnectVPN;

mod lsdir;
mod vpn;
mod actions;

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
    setup_logger().unwrap();

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
                let normalized_message = message_txt.trim_start().to_lowercase();
                //TODO implement pattern matching using my custom commands
                if normalized_message.starts_with(lsdir::LS_DIR_CMD) {
                    let list_dir = ListDir { msg: normalized_message.to_owned() };
                    let results = list_dir.execute();
                    client.send_privmsg(&channel, results).unwrap();
                    //TODO implement pattern matching using my custom commands
                } else if normalized_message.starts_with(vpn::VPN_CONNECT_CMD) {
                    let connect_vpn = ConnectVPN { msg: normalized_message.to_owned() };
                    let result = connect_vpn.execute();
                    client.send_privmsg(&channel, result).unwrap();
                } else {
                    client.send_privmsg(&channel, "Unrecognized command: ".to_owned() + message_txt.as_ref()).unwrap();
                }
            }
            _ => (),
        }
    }

    Ok(())
}
