# This bot was made to clone messages from a channel to other chats
Note: it copies the messages from a channel post and sends them to chats subscribed via the bot, it doesn't forward the messages.

There are 3 commands available
1. /help - This command provides a brief description of the other available commands.
2. /addchat - This command allows you to add a chat to the subscriber list of a channel publisher. When this command is used, all posts from the channel will be copied to the specified chat.  
To use this command, type: /addchat FROM TO, where "FROM" is the ID or username (@username) of the publisher channel, and "TO" is the ID or username of the chat where all the channel posts will be sent.  
Example usage: /addchat @mychannel @mychannelclone.  
Example: /addchat @mychannel @mychannelclone  
**Please note that both the bot and the user this command must be administrators of both the channel and the chat for it to work.**
3. /id - By default, this command returns the ChatID of the current chat. However, if you reply to a message that was forwarded from another chat, it will return the ChatID of the chat from which the message was originally forwarded.  

# Important
You must have a channels.json inside the config folder in the root of the executable
The json file follows the structure used by the bot.
```json
[
    {
        "from": "@mychannel",
        "to": ["@mychannelclone", "@mychannelclone2"]
    },
    {
        "from": "@myotherchannel",
        "to": ["@myotherchannelclone", "@myotherchannelclone2"]
    }
]
```

# Compile and Run
Run the executable with the flags --dev-id providing your telegram user id, and --token with your bot token obtained from @BotFather.
Example:
```sh
./path/to/executable --dev-id 1000123 --token 123456:ABCDEFGabcdef
```
or with cargo:
```sh
cargo run --release -- --dev-id 1000123 --token 123456:ABCDEFGabcdef
```
