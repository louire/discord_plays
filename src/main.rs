use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::gateway::GatewayIntents;
use songbird::SerenityInit;

#[group]
#[commands(join, leave, play)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} está conectado!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let token = "TOKEN";

    let intents = GatewayIntents::non_privileged() 
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error al crear el cliente");

    if let Err(why) = client.start().await {
        println!("Error al iniciar el cliente: {:?}", why);
    }
}

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
            msg.reply(ctx, "Primero únete a un canal de voz!").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let _handler = manager.join(guild_id, connect_to).await;
    msg.channel_id.say(&ctx.http, "¡Me uní al canal de voz!").await?;
    
    Ok(())
}

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
        msg.channel_id.say(&ctx.http, "¡Adiós!").await?;
    } else {
        msg.reply(ctx, "No estoy en un canal de voz").await?;
    }

    Ok(())
}

#[command]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let url = msg.content.split_whitespace().nth(1).unwrap_or("");
        if url.is_empty() {
            msg.channel_id.say(&ctx.http, "Por favor, proporciona una URL de YouTube").await?;
            return Ok(());
        }

        msg.channel_id.say(&ctx.http, "🎵 Procesando...").await?;
        
        let source = match songbird::ytdl_search(url)
            .await
            .map_err(|e| {
                println!("Error en ytdl_search: {:?}", e);
                e
            }) {
            Ok(source) => source,
            Err(why) => {
                println!("Error detallado al obtener el audio: {:?}", why);
                msg.channel_id.say(&ctx.http, format!("❌ Error al reproducir el audio: {:?}", why)).await?;
                return Ok(());
            },
        };
        
        println!("✅ Fuente de audio obtenida correctamente");

        let track_handle = handler.play_source(source);
        println!("🎵 Iniciando reproducción");
        
        let _ = track_handle.set_volume(0.5);
        
        msg.channel_id.say(&ctx.http, "▶️ ¡Reproduciendo! (Volumen: 50%)").await?;
        
        // Manejador de eventos de la pista
        let send_http = ctx.http.clone();
        let chan_id = msg.channel_id;
        
        track_handle.add_event(
            songbird::Event::Track(songbird::TrackEvent::End),
            songbird::TrackEventHandler::new(move |_| {
                let _ = chan_id.say(&send_http, "🎵 Canción terminada");
                Box::pin(async move {})
            }),
        );
    } else {
        msg.channel_id.say(&ctx.http, "¡Primero necesito unirme a un canal de voz! Usa !join").await?;
    }

    Ok(())
}