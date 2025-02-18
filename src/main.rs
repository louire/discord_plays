use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::gateway::GatewayIntents;
use songbird::SerenityInit;

// Command group definition for music bot commands
#[group]
#[commands(join, leave, play)]
struct General;

// Basic event handler for bot functionality
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

// Main function to set up and start the Discord bot
#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let token = "PLACE_YOUR_TOKEN_HERE";

    // Set up required gateway intents for the bot
    let intents = GatewayIntents::non_privileged() 
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

// Command to join a voice channel
#[command]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.reply(ctx, "First join a voice channel!").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let _handler = manager.join(guild_id, connect_to).await;
    msg.channel_id.say(&ctx.http, "Joined the voice channel!").await?;
    
    Ok(())
}

// Command to leave the current voice channel
#[command]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            msg.channel_id.say(&ctx.http, format!("Error: {:?}", e)).await?;
        }
        msg.channel_id.say(&ctx.http, "Goodbye!").await?;
    } else {
        msg.reply(ctx, "Not in a voice channel").await?;
    }

    Ok(())
}

// Command to play audio from a YouTube URL
#[command]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        // Extract URL from message
        let url = msg.content.split_whitespace().nth(1).unwrap_or("");
        if url.is_empty() {
            msg.channel_id.say(&ctx.http, "Please provide a YouTube URL").await?;
            return Ok(());
        }

        msg.channel_id.say(&ctx.http, "🎵 Processing...").await?;
        
        // Attempt to load the audio source
        let source = match songbird::ytdl_search(url)
            .await
            .map_err(|e| {
                println!("ytdl_search error: {:?}", e);
                e
            }) {
            Ok(source) => source,
            Err(why) => {
                println!("Detailed audio error: {:?}", why);
                msg.channel_id.say(&ctx.http, format!("❌ Error playing audio: {:?}", why)).await?;
                return Ok(());
            },
        };
        
        println!("✅ Audio source obtained successfully");

        // Set up audio playback
        let track_handle = handler.play_source(source);
        println!("🎵 Starting playback");
        
        // Set default volume to 50%
        let _ = track_handle.set_volume(0.5);
        
        msg.channel_id.say(&ctx.http, "▶️ Now playing! (Volume: 50%)").await?;
        
        // Set up track event handler
        let send_http = ctx.http.clone();
        let chan_id = msg.channel_id;
        
        track_handle.add_event(
            songbird::Event::Track(songbird::TrackEvent::End),
            songbird::TrackEventHandler::new(move |_| {
                let _ = chan_id.say(&send_http, "🎵 Track finished");
                Box::pin(async move {})
            }),
        );
    } else {
        msg.channel_id.say(&ctx.http, "First I need to join a voice channel! Use !join").await?;
    }

    Ok(())
}