use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use chrono::{Duration, Utc};
use lazy_static::lazy_static;
use teloxide::{
    requests::Requester,
    types::{
        InputFile, InputMedia, InputMediaAnimation, InputMediaAudio, InputMediaDocument,
        InputMediaPhoto, InputMediaVideo, MediaKind,
    },
    Bot,
};

#[derive(Clone)]

pub struct ScheduleMediaGroup {
    pub media_group_id: String,
    pub media_group: Vec<MediaKind>,
    pub schedule_millis: i64,
    pub send_to: Vec<String>,
}

impl ScheduleMediaGroup {
    pub fn new(
        media_group_id: String,
        media_group: Vec<MediaKind>,
        send_to: Vec<String>,
        schedule_millis: i64,
    ) -> Self {
        Self {
            media_group,
            media_group_id,
            send_to,
            schedule_millis,
        }
    }
    pub fn insert(
        schedule: &mut VecDeque<Self>,
        media: MediaKind,
        media_group_id: String,
        send_to: Vec<String>,
    ) {
        match schedule
            .iter_mut()
            .find(|scheduled| scheduled.media_group_id == media_group_id)
        {
            Some(found) => {
                found.media_group.push(media);
            }
            None => {
                //
                let schedule_milis =
                    Utc::now().timestamp_millis() + Duration::seconds(5).num_milliseconds();
                let new_schedule = Self::new(media_group_id, vec![media], send_to, schedule_milis);
                schedule.push_back(new_schedule);
            }
        }
    }
    #[allow(clippy::redundant_async_block)]
    pub async fn send(&self, tg: &Bot) {
        let input_media_group =
            &self
                .media_group
                .iter()
                .filter_map(|media_kind| match &media_kind {
                    MediaKind::Animation(media) => {
                        let mut anime = InputMediaAnimation::new(InputFile::file_id(
                            media.animation.file.id.clone(),
                        ))
                        .caption_entities(media.caption_entities.clone());

                        anime.caption = media.caption.clone();
                        Some(InputMedia::Animation(anime))
                    }
                    MediaKind::Audio(media) => {
                        let mut audio =
                            InputMediaAudio::new(InputFile::file_id(media.audio.file.id.clone()))
                                .caption_entities(media.caption_entities.clone());

                        audio.caption = media.caption.clone();
                        Some(InputMedia::Audio(audio))
                    }
                    MediaKind::Document(media) => {
                        let mut document = InputMediaDocument::new(InputFile::file_id(
                            media.document.file.id.clone(),
                        ))
                        .caption_entities(media.caption_entities.clone());

                        document.caption = media.caption.clone();
                        Some(InputMedia::Document(document))
                    }
                    MediaKind::Photo(media) => {
                        let mut photo = InputMediaPhoto::new(InputFile::file_id(
                            media.photo.last().unwrap().file.id.clone(),
                        ))
                        .caption_entities(media.caption_entities.clone());

                        photo.caption = media.caption.clone();
                        Some(InputMedia::Photo(photo))
                    }
                    MediaKind::Video(media) => {
                        let mut video =
                            InputMediaVideo::new(InputFile::file_id(media.video.file.id.clone()))
                                .caption_entities(media.caption_entities.clone());

                        video.caption = media.caption.clone();
                        Some(InputMedia::Video(video))
                    }
                    MediaKind::VideoNote(media) => {
                        let video = InputMediaVideo::new(InputFile::file_id(
                            media.video_note.file.id.clone(),
                        ));

                        Some(InputMedia::Video(video))
                    }
                    _ => None,
                });

        let msgs = self
            .send_to
            .iter()
            .map(|to| tg.send_media_group(to.to_string(), input_media_group.clone()));
        log::info!("Cloning messages to {} targets.", msgs.len());
        for msg in msgs {
            tokio::spawn(async { msg.await });
        }
    }
}

lazy_static! {
    pub static ref SCHEDULE_MEDIA_GROUP: Arc<Mutex<VecDeque<ScheduleMediaGroup>>> =
        Arc::new(Mutex::new(VecDeque::new()));
}
