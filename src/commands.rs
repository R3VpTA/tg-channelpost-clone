use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Commands:")]
pub enum MyCommands {
    #[command(description = "Display this text.")]
    Help,

    #[command(
        description = "Add chat to listen for channel posts. Pass the channel you want to listen and to where it's gonna copy the messages.\nUsage: /addchat @channelusernameorid chatusernameorid\nExample: /addchat @mychannel @mychannelclone\nWith id: /addchat -123456789s @mychannelclone",
        parse_with = "split"
    )]
    AddChat { from: String, to: String },

    #[command(
        description = "This command retrieves the current chat's ID. If the message is a reply to a forwarded message, it returns the Chat ID from the original source of the forwarded message."
    )]
    Id,
}
