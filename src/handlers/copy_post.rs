use std::{
    borrow::BorrowMut,
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use teloxide::{
    prelude::*,
    types::{Message, MessageKind},
};

use crate::{
    bot::HandlerResult,
    channel_storage::ChannelStorage,
    schedule_media_groups::{ScheduleMediaGroup, SCHEDULE_MEDIA_GROUP},
};

pub async fn copy_post(
    tg: Bot,
    msg: Message,
    channels: Arc<Mutex<Vec<ChannelStorage>>>,
) -> HandlerResult {
    let target = {
        let channels = channels.lock().unwrap();

        let id = msg.chat.id.clone().to_string();
        let user_or_id = msg.chat.username().unwrap_or(id.as_str());

        let targets = channels.iter().find(|channel| channel.from == user_or_id);
        if let Some(target) = targets {
            target.to_owned()
        } else {
            return Ok(());
        }
    };

    match msg.media_group_id() {
        None => {
            for to in target.to {
                tg.copy_message(to, msg.chat.id, msg.id).await?;
            }
            return Ok(());
        }
        Some(media_group_id) => {
            if let MessageKind::Common(m) = msg.kind.clone() {
                let mut mutex = SCHEDULE_MEDIA_GROUP.lock().unwrap();
                let schedule = mutex.borrow_mut();

                ScheduleMediaGroup::insert(
                    schedule,
                    m.media_kind,
                    media_group_id.to_string(),
                    target.to,
                );

                drop(mutex);
            }
        }
    }

    Ok(())
}
