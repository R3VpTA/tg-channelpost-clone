use std::sync::{Arc, Mutex};

use thiserror::Error;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct ChannelStorage {
    pub from: String,
    pub to: Vec<String>,
}

#[derive(Error, Debug)]
pub enum ChannelStoreError {
    #[error("Error occurred while reading file:\n\n {0}")]
    Reading(#[from] std::io::Error),
    #[error("Error occurred while serializing or deserializing json:\n\n {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Duplicated entry detected. The channel you are trying to add is already listening to channel posts from the channel you're trying to clone")]
    DuplicateTo,
}

impl ChannelStorage {
    pub fn new(from: impl ToString, to: Vec<String>) -> Self {
        Self {
            from: from.to_string(),
            to,
        }
    }
    pub fn load_all() -> Result<Vec<Self>, ChannelStoreError> {
        let contents = std::fs::read_to_string("./config/channels.json")?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn add(
        channels: &Arc<Mutex<Vec<Self>>>,
        from: &str,
        to: &str,
    ) -> Result<(), ChannelStoreError> {
        {
            let mut mutex = channels.lock().unwrap();
            match mutex.iter_mut().find(|channel| channel.from == from) {
                Some(channel) => {
                    if channel.to.contains(&to.to_string()) {
                        return Err(ChannelStoreError::DuplicateTo);
                    } else {
                        channel.to.push(to.to_string());
                    }
                }
                None => {
                    let new_channel = ChannelStorage::new(from, vec![to.to_string()]);
                    mutex.push(new_channel);
                }
            }
            let value: Vec<ChannelStorage> = mutex.to_owned();
            drop(mutex);
            let contents = serde_json::to_string_pretty(&value)?;
            Ok(std::fs::write("./config/channels.json", contents)?)
        }
    }
}
