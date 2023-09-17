use bot::start_bot;
mod bot;
mod channel_storage;
mod cli;
mod commands;
mod handlers;
mod schedule_media_groups;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    start_bot().await;
}
