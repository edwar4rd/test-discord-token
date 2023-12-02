// check if discord bot token is valid
// and list its information

use std::env::{self, args};

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::TypeMapKey;
use serenity::prelude::*;

struct Handler;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = std::sync::Arc<serenity::all::ShardManager>;
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        // Bot is connected: print out some simple information then shutdown the bot
        println!("Bot connected!\n");

        println!("Info:");
        println!("\tName: {}", ready.user.name);
        println!("\tIs bot: {}", ready.user.bot);
        println!("\tID: {}", ready.user.id);
        println!("\tGuild count: {}\n", ready.guilds.len());

        for guild in ready.guilds.iter().take(20) {
            let guild = ctx.http.get_guild_preview(guild.id).await.unwrap();
            println!(
                "\tguild:\n\t\t{}\n\t\t{} member(s)\n",
                guild.name, guild.approximate_member_count
            );
        }

        // Shutdown the bot with a reference of the shard manager
        match ctx.data.read().await.get::<ShardManagerContainer>() {
            Some(v) => v,
            None => {
                eprintln!("Failed stopping the bot...");
                return;
            }
        }
        .shutdown_all()
        .await;
        println!("Bot should be stopped...");
    }
}

#[tokio::main]
async fn main() {
    let token = if env::args().len() > 1 {
        args().nth(1).unwrap()
    } else {
        env::var("DISCORD_TOKEN")
            .expect("Expected a token in the environment or from command line argument!")
    };

    let mut client = Client::builder(&token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    client
        .data
        .write()
        .await
        .insert::<ShardManagerContainer>(client.shard_manager.clone());

    if let Err(err) = client.start().await {
        println!("Client error: {err:?}");
    }
}
