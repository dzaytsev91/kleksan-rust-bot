use teloxide::{prelude::*, utils::command::BotCommands, types::ReplyParameters};
use chrono::{Datelike, Utc};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

/// These commands are supported:
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Display this text.
    #[command(aliases = ["h", "?"])]
    Help,
    /// Handle a username.
    #[command(alias = "w")]
    Where,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Where => {
            let dt = Utc::now();
            if dt.day() % 2 == 0 {
                bot.send_message(msg.chat.id, "Сегодня колоть вправо").reply_parameters(ReplyParameters::new(msg.id)).await?
            } else {
                bot.send_message(msg.chat.id, "Сегодня колоть влево").reply_parameters(ReplyParameters::new(msg.id)).await?
            }
        }
    };
    Ok(())
}
