use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
};

use crate::{
    channel_storage::ChannelStorage,
    cli::CLI,
    commands::MyCommands,
    handlers::{commands::command_handler, copy_post::copy_post},
    schedule_media_groups::{ScheduleMediaGroup, SCHEDULE_MEDIA_GROUP},
};
use teloxide::prelude::*;

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn start_bot() {
    let channels = Arc::new(Mutex::new(ChannelStorage::load_all().unwrap()));
    let tg = Bot::new(&CLI.token);

    let bot = tg.clone();
    tokio::spawn(async {
        resolve_media_group(bot).await;
    });

    log::info!("Bot {} is running", tg.get_me().await.unwrap().full_name());

    let handler = dptree::entry()
        .branch(Update::filter_channel_post().endpoint(copy_post))
        .branch(
            Update::filter_message()
                .filter_command::<MyCommands>()
                .endpoint(command_handler),
        );
    Dispatcher::builder(tg, handler)
        .dependencies(dptree::deps![channels])
        .build()
        .dispatch()
        .await;
}

async fn resolve_media_group(tg: Bot) {
    loop {
        let elements = {
            let mut schedule = SCHEDULE_MEDIA_GROUP.lock().unwrap();
            let schedules = schedule.borrow_mut();

            let now = chrono::Utc::now().timestamp_millis();

            let mut elements: Vec<ScheduleMediaGroup> = vec![];
            for element in schedules.clone().iter() {
                if now < element.schedule_millis {
                    break;
                }
                elements.push(element.to_owned());
                schedules.pop_front();
            }
            drop(schedule);
            elements
        };
        for element in elements {
            element.send(&tg).await;
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
