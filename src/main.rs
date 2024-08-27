use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::{gateway::Ready, id::{GuildId, ChannelId}},
    prelude::*,
};
use songbird::{input::{self, File}, SerenityInit};
use std::env;
use dotenv::dotenv;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Define the voice channel ID to join
        let guild_id = GuildId::new(1209607225035718666);
        let channel_id = ChannelId::new(1209607226000547884);

        // Get the Songbird voice client
        let manager = songbird::get(&ctx).await
            .expect("Songbird Voice client placed in at initialization.").clone();

        // Join the voice channel
        if let Ok(_handler) = manager.join(guild_id, channel_id).await {
            println!("Bot joined the voice channel");

            // Start playing music after joining
            play_music(&ctx, guild_id).await;
        } else {
            println!("Failed to join the voice channel");
        }
    }
}

async fn play_music(ctx: &Context, guild_id: GuildId) {
    let manager = songbird::get(ctx).await.expect("Songbird Voice client placed at initialization").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        loop {
            for track_name in &["track0.mp3", "track1.mp3", "track2.mp3"] {
                let source = input::File::new(format!("./tracks/{}", track_name));

                

            }
        }
    } else {
        println!("Handler not found for the given guild ID");
    }
}



#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_VOICE_STATES;

    // Register songbird with the client
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
