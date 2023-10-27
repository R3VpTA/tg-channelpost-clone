use std::sync::{Arc, Mutex};

use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{bot::HandlerResult, channel_storage::ChannelStorage, cli::CLI, commands::MyCommands};

pub async fn command_handler(
    tg: Bot,
    msg: Message,
    cmd: MyCommands,
    channels: Arc<Mutex<Vec<ChannelStorage>>>,
) -> HandlerResult {
    match cmd {
        MyCommands::Help => {
            tg.send_message(msg.chat.id, MyCommands::descriptions().to_string())
                .await?;
        }
        MyCommands::AddChat { from, to } => {
            add_chat(tg, &msg, channels, from, to).await?;
        }
        MyCommands::Id => get_id(tg, &msg).await?,
    }
    Ok(())
}

async fn add_chat(
    tg: Bot,
    msg: &Message,
    channels: Arc<Mutex<Vec<ChannelStorage>>>,
    from: String,
    to: String,
) -> HandlerResult {
    if from == to {
        tg.send_message(
            msg.chat.id,
            "You cannot clone messages to the same chat from which they originate",
        )
        .await?;
        return Ok(());
    }

    let user_from = if let Some(user) = msg.from() {
        user
    } else {
        return Ok(());
    };

    let from_chat = tg.get_chat_member(from.clone(), user_from.id);
    let to_chat = tg.get_chat_member(to.clone(), user_from.id);

    match (from_chat.await, to_chat.await) {
        (Ok(from_c), Ok(to_c)) => {
            if !from_c.is_privileged() {
                tg.send_message(msg.chat.id, format!("You must be an admin or owner of the provided chat, but you are neither in the channel ({}) from which you are attempting to clone the messages.", from)).await?;
                return Ok(());
            }
            if !to_c.is_privileged() {
                tg.send_message(msg.chat.id, format!("To clone messages to the provided chat, you must be an admin or owner, but you are neither in the channel ({}).",to)).await?;
                return Ok(());
            }
        }
        (Ok(_), Err(_)) => {
            tg.send_message(msg.chat.id, format!("Could not find chat \"{}\"", from))
                .await?;
            return Ok(());
        }
        (Err(_), Ok(_)) => {
            tg.send_message(msg.chat.id, format!("Could not find chat \"{}\"", to))
                .await?;
            return Ok(());
        }
        (Err(e1), Err(e2)) => {
            log::warn!("Not found chat 1: \n{e1}");
            log::warn!("Not found chat 2: \n{e2}");
            tg.send_message(
                msg.chat.id,
                format!("Could not find chat \"{}\" and \"{}\".", from, to),
            )
            .await?;
            return Ok(());
        }
    }

    let from = tg.get_chat(from).await?.id.to_string();
    let to = tg.get_chat(to).await?.id.to_string();

    let channels_ = {
        let mutex = channels.lock().unwrap();
        let channels_: Vec<ChannelStorage> = mutex.to_owned();
        drop(mutex);
        channels_
    };
    if channels_
        .iter()
        .any(|channel| channel.from == from && channel.to.contains(&to))
    {
        tg.send_message(msg.chat.id, "Chat is already registered.")
            .reply_to_message_id(msg.id)
            .await?;
        return Ok(());
    }

    match ChannelStorage::add(&channels, &from, &to) {
        Ok(_) => {
            let new_chat_msg =
                format!("Chat \"{to}\" added to listen to channel posts of channel \"{from}\"");
            log::info!("{new_chat_msg}");
            tg.send_message(msg.chat.id, new_chat_msg).await?;
        }

        Err(e) => {
            // Respond to the user that an error occurred
            tg.send_message(msg.chat.id, format!("Failed to add chat, contact the <a href=\"tg://user?id={}\">developer</a> to know the cause.", CLI.dev_id)).parse_mode(teloxide::types::ParseMode::Html).await?;

            // Notify dev of the error
            tg.send_message(ChatId(CLI.dev_id), format!("An error occurred while trying to add chat {to} to clone messages from channel {from}, details: \n\n{e}")).await?;
            return Ok(());
        }
    }
    Ok(())
}

async fn get_id(tg: Bot, msg: &Message) -> HandlerResult {
    let id = msg
        .reply_to_message()
        .unwrap_or(msg)
        .forward_from_chat()
        .unwrap_or(&msg.chat)
        .id;

    tg.send_message(msg.chat.id, format!("ID: {id}")).await?;

    Ok(())
}
