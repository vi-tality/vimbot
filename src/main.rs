mod commands;
use commands::{math::*, meta::*};

use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group
        }
    },
    prelude::*,
};

#[group]
#[commands(multiply, divide, ping, iwantvi, iwantneovim, iwantemacs, idontwantvi, idontwantemacs, idontwantneovim, help, q, ynix, gDots, jDots, zDots, kDots, pins, neovitality, insultCindy, yflakes, ygentoo, bDots, gytisPowers, test_embed )]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(":")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
