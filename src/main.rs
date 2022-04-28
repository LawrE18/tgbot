use teloxide::{prelude::*, utils::command::BotCommands};

use std::error::Error;
extern crate rand;
extern crate ed25519_dalek;
extern crate hex;

use rand::rngs::OsRng;
use ed25519_dalek::Keypair;
use ed25519_dalek::{PublicKey};
use ed25519_dalek::{PUBLIC_KEY_LENGTH};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "create wallet.")]
    CreateWallet,
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::CreateWallet => {
            let mut csprng = OsRng{};
            let keypair: Keypair = Keypair::generate(&mut csprng);
            let public_key: PublicKey = keypair.public;
            let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = public_key.to_bytes();
            let public_key_hex = hex::encode(public_key_bytes);
            bot.send_message(message.chat.id, format!("Pub key is: {}", public_key_hex)).await?
        }
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string()).await?
        }
        Command::Username(username) => {
            bot.send_message(message.chat.id, format!("Your username is @{username}.")).await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                message.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
                .await?
        }
    };

    Ok(())
}
