//! Requires the 'framework' feature flag be enabled in your project's
//! `Cargo.toml`.
//!
//! This can be enabled by specifying the feature in the dependency section:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["framework", "standard_framework"]
//! ```
mod commands;

use log::{error, info};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    model::{
        event::ResumedEvent,
        gateway::Ready,
        guild::Member,
        id::{ChannelId, GuildId, RoleId},
        user::User,
    },
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};

use commands::{math::*, meta::*, owner::*};
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
    fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, mut member: Member) {
        let user_id = member.user_id();
        let _ = ChannelId(649328791692247053).say(&ctx.http, format!(r#"**`:normal iHi `**  {}  **`, you are our`**`<C-r>=GetDiscordUsers("{:?}")<CR>`**`user in this community of vim enthusiasts.`**"#, member.mention(), user_id.as_u64()));

        let channel_id = env::var("CHANNEL_ID").expect("Expected a CHANNEL_ID in the environment");
        if let Err(e) = member.add_role(&ctx, RoleId(channel_id)) {
            error!("Unable to add roles to {}: {}", member.display_name(), e);
        }
    }

    fn guild_member_removal(
        &self,
        ctx: Context,
        _guild: GuildId,
        user: User,
        _member_data_if_available: Option<Member>,
    ) {
        let _ = ChannelId(649328791692247053).say(
            &ctx.http,
            format!(
                "Oof. {} started using emacs. What a gnuisance D:",
                user.mention()
            ),
        );
    }
}

group!({
    name: "general",
    options: {},
    commands: [multiply, ping, iwantvi, iwantneovim, iwantemacs, idontwantvi, idontwantemacs, idontwantneovim, help, quit]
});

fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    kankyo::load();

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix("~"))
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }

    //println!("join link: https://discordapp.com/api/oauth2/authorize?client_id={:}&permissions=0&scope=bot", client.user);
}
