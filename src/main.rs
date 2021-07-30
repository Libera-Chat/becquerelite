use irc::client::prelude::*;
use futures::prelude::*;
use tokio::time::{sleep_until, Duration, Instant};
use chrono::prelude::*;

use std::convert::TryInto;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let channel = "#libera-clock".to_string();
    let config = Config {
        nickname: Some("becquerelite".to_owned()),
        username: Some("becquerelite".to_owned()),
        server: Some("irc.libera.chat".to_owned()),
        channels: vec![channel.clone()],
        ..Config::default()
    };

    let mut client = Client::from_config(config).await?;
    let mut stream = client.stream()?;
    client.identify()?;

    let timeout = sleep_until(Instant::now() + Duration::from_secs(100));
    tokio::pin!(timeout);

    loop {
        let now = Utc::now();
        let secs_to_go: u64 = (10 - now.timestamp()%10).try_into()?;
        timeout.as_mut().reset(Instant::now() + Duration::from_secs(secs_to_go));
        tokio::select! {
            _ = &mut timeout => {
                let _ = client.send_privmsg(
                    &channel,
                    Utc::now().format("It is %A, %Y-%m-%d %H:%M:%SZ (Libera Standard Time) at the beep. Beep.")
                );
            }

            Some(msg) = stream.next() => {
                println!("{:?}", msg);
            }
        }
    }
}
