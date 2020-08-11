use irc::client::prelude::*;
use futures::prelude::*;
use std::process::Command;
use std::io::{self, Write};

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
        // TODO
        // - react on particular command messages on irc 
        // - decrypt message content (alg tbd)
        // - check if vpn connection is open and if not 
        //    use decrypted token to initiate vpn connection (all via shell Command::new())
        //    ircayc must not use 'rootish' permissions
        // - open ssh tunnel to preconfigured host(.ssh/config + public key auth)
        // 
        // other things: 
        // - systemd service
        // - all configuration at $HOME/.ircyayc/config  with proper rights (consider encryption)


        // this is just a PoC
        let dirs = Command::new("ls").output().expect("failed to execute process");
       
        println!("status: {}", dirs.status);
        io::stdout().write_all(&dirs.stdout).unwrap();
        io::stderr().write_all(&dirs.stderr).unwrap();
        print!("{}", message);
    }

    Ok(())
}