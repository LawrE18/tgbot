mod crypto;
use teloxide::{
    dispatching::dialogue::InMemStorage, prelude::*, types::Update, utils::command::BotCommands,
};

use serde_json::json;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
#[derive(Clone)]
pub enum State {
    Start,
    ReceiveTo,
    ReceiveAmount { to: String },
    Sign { to: String, amount: u32 },
}

impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env().auto_send();
    let cloned_bot = bot.clone();

    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(
            dptree::case![State::Start]
                .branch(dptree::entry().filter_command::<Command>().endpoint(start)),
        )
        .branch(dptree::case![State::ReceiveTo].endpoint(receive_to))
        .branch(dptree::case![State::ReceiveAmount { to }].endpoint(signing));

    Dispatcher::builder(cloned_bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "create wallet.")]
    CreateWallet,
    #[command(description = "get address.")]
    GetMyAddress,
    #[command(description = "sign tx.")]
    SignTx,
}

async fn start(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
    dialogue: MyDialogue,
) -> HandlerResult {
    match command {
        Command::Help => help_message(bot, message).await?,
        Command::CreateWallet => create_wallet_message(bot, message).await?,
        Command::GetMyAddress => get_address_message(bot, message).await?,
        Command::SignTx => {
            dialogue.update(State::ReceiveTo).await?;
            signtx_message(bot, message).await?
        }
    };

    Ok(())
}

async fn help_message(bot: AutoSend<Bot>, msg: Message) -> Result<Message, teloxide::RequestError> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await
}

async fn create_wallet_message(
    bot: AutoSend<Bot>,
    msg: Message,
) -> Result<Message, teloxide::RequestError> {
    let pub_bytes = crypto::gen_key_pair(msg.chat.id.0);
    bot.send_message(msg.chat.id, format!("pub key: {}", hex::encode(pub_bytes)))
        .await
}

async fn get_address_message(
    bot: AutoSend<Bot>,
    msg: Message,
) -> Result<Message, teloxide::RequestError> {
    bot.send_message(
        msg.chat.id,
        format!("address: {}", crypto::get_address(msg.chat.id.0)),
    )
    .await
}

async fn signtx_message(
    bot: AutoSend<Bot>,
    msg: Message,
) -> Result<Message, teloxide::RequestError> {
    bot.send_message(msg.chat.id, "To whom".to_string()).await
}

async fn receive_to(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "Amount?").await?;
            dialogue
                .update(State::ReceiveAmount { to: text.into() })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}

async fn signing(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    to: String,
) -> HandlerResult {
    match msg.text().map(|text| text.parse::<u32>()) {
        Some(Ok(amount)) => {
            let from = msg.chat.username();
            let tx = json!({
                "from": from,
                "to": to,
                "amount": amount,
            });
            let sign = hex::encode(crypto::sign(msg.chat.id.0, tx.to_string()));
            let signed_tx = json!({
                "from": from,
                "to": to,
                "amount": amount,
                "sign": sign,
            });
            dialogue.reset().await?;
            bot.send_message(msg.chat.id, signed_tx.to_string()).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me a number.").await?;
        }
    }

    Ok(())
}
