use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText, Update,
    },
    utils::command::BotCommands,
};

use serde_json::json;
type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub mod mongo;
pub use mongo::crypto::*;
pub mod traits;
use std::{str, vec};
pub use traits::*;

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

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<State>, State>()
                .branch(
                    dptree::case![State::Start].branch(
                        dptree::entry()
                            .filter_command::<Command>()
                            .endpoint(message_handler),
                    ),
                )
                .branch(dptree::case![State::ReceiveTo].endpoint(receive_to))
                .branch(dptree::case![State::ReceiveAmount { to }].endpoint(signing)),
        )
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(cloned_bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}

/// Creates a keyboard made by buttons in a big column.
fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let sig_schemes = ["Ed25519", "Sr25519"];

    for scheme in sig_schemes.chunks(1) {
        let row = scheme
            .iter()
            .map(|&version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
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

async fn message_handler(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
    dialogue: MyDialogue,
) -> HandlerResult {
    match command {
        Command::Help => help_message(bot, message).await?,
        Command::CreateWallet => {
            //let keyboard = make_keyboard();
            //bot.send_message(message.chat.id, "Signature schemes")
            //    .reply_markup(keyboard)
            //    .await?;
            create_wallet_message(bot, message).await?
        }
        Command::GetMyAddress => get_address_message(bot, message).await?,
        Command::SignTx => {
            dialogue.update(State::ReceiveTo).await?;
            signtx_message(bot, message).await?
        }
    };

    Ok(())
}

async fn inline_query_handler(q: InlineQuery, bot: AutoSend<Bot>) -> HandlerResult {
    let choose_scheme = InlineQueryResultArticle::new(
        "0",
        "Chose signature scheme",
        InputMessageContent::Text(InputMessageContentText::new("Signature schemes:")),
    )
    .reply_markup(make_keyboard());

    bot.answer_inline_query(q.id, vec![choose_scheme.into()])
        .await?;

    Ok(())
}

async fn callback_handler(q: CallbackQuery, bot: AutoSend<Bot>) -> HandlerResult {
    if let Some(scheme) = q.data {
        let mut text = format!("You chose: {scheme}");

        match q.message {
            Some(Message { id, chat, .. }) => {
                let user = Sr25519 { id_: chat.id.0 };
                user.generate_keypairs().expect("error in gen keypairs");
                if let Ok(t) = user.get_public() {
                    text = hex::encode(t);
                }
                //create_wallet_message(bot, msg).await?;
                bot.edit_message_text(chat.id, id, text).await?;
            }
            None => {
                if let Some(id) = q.inline_message_id {
                    bot.edit_message_text_inline(id, text).await?;
                }
            }
        }

        log::info!("You chose: {}", scheme);
    }

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
    let keyboard = make_keyboard();
    bot.send_message(msg.chat.id, "Signature schemes:")
        .reply_markup(keyboard)
        .await
}

async fn get_address_message(
    bot: AutoSend<Bot>,
    msg: Message,
) -> Result<Message, teloxide::RequestError> {
    bot.send_message(
        msg.chat.id,
        format!("address: {}", get_address(msg.chat.id.0)),
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
            let sign = hex::encode(sign(msg.chat.id.0, tx.to_string()));
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
