use clap::Parser;
use itertools::Itertools;
use serde::Deserialize;
use twitch_irc::message::{IRCMessage, ServerMessage};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The channel login of the Twitch channel to fetch. E.g. `pajlada` or `riotgames`
    #[arg(value_parser = validate_channel_login)]
    channel_login: String,
}

fn validate_channel_login(channel_login: &str) -> Result<String, String> {
    match twitch_irc::validate::validate_login(channel_login) {
        Ok(()) => Ok(channel_login.to_owned()),
        Err(e) => Err(format!("{}", e)),
    }
}

#[derive(Deserialize)]
struct ApiResponse {
    messages: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let http_client = reqwest::blocking::Client::builder()
        .user_agent(user_agent)
        .build()
        .unwrap();

    let url = format!(
        "https://recent-messages.robotty.de/api/v2/recent-messages/{}",
        urlencoding::encode(&cli.channel_login)
    );
    let res: ApiResponse = http_client
        .get(url)
        .send()
        .expect("HTTP request failed")
        .error_for_status()
        .expect("Server returned an error code")
        .json()
        .expect("Server returned data in invalid format");

    let messages_text = res
        .messages
        .iter()
        .map(|s| IRCMessage::parse(s).expect("Malformed IRC message received from server"))
        .map(|m| ServerMessage::try_from(m).expect("Malformed IRC message received from server"))
        .filter_map(|m| match m {
            ServerMessage::Privmsg(msg) => Some(msg),
            _ => None,
        })
        .map(|m| format!("{}: {}", &m.sender.name, m.message_text))
        .join(LINE_ENDING);
    println!("{}", messages_text);
}
