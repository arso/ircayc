use futures::prelude::*;
use irc::client::prelude::*;

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    let config = Config {
        nickname: Some("asochatst".to_owned()),
        server: Some("chat.freenode.net".to_owned()),
        channels: vec!["#asochatst".to_owned()],
        use_tls: Some(false),
        ..Default::default()
    };
    
    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    let sender = client.sender();

    while let Some(message) = stream.next().await.transpose()? {
        print!("{}", message);

        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                sender.send_privmsg(target, "hello")?;
            }
            _ => (),
        }
    }

    Ok(());
}
