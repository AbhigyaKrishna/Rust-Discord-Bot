use serenity::client::Context;
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::guild::Change::ChannelId;
use serenity::model::prelude::Message;
use songbird::{Event, TrackEvent};
use songbird::input::Restartable;
use crate::music::{get_songbird_manager, SongEndNotifier};

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut arg: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|vc| vc.channel_id);

    let Some(id) = channel_id else {
        msg.reply(ctx, "Not in voice channel").await?;
        return Ok(())
    };

    let manager = get_songbird_manager(ctx).await;

    if let Some(h) = manager.get(guild_id) {
        let mut handler = h.lock().await;
        if handler.current_channel().map(|x| x.0) != channel_id.map(|x| x.0) {
            let _ = handler.join(id).await;
        }
    } else {
        manager.join(guild_id, id).await;
    }

    let handler = manager.get(guild_id).unwrap();
    let mut handler = handler.lock().await;

    let term = arg.rest();

    match Restartable::ytdl_search(term, true).await {
        Ok(src) => {
            let queue = !handler.queue().is_empty();
            let song = handler.enqueue_source(src.into());
            let send_http = ctx.http.clone();
            let channel_id = msg.channel_id;

            let _ = song.add_event(
                Event::Track(TrackEvent::End),
                SongEndNotifier {
                    channel_id,
                    http: send_http
                }
            );

            if queue {
                msg.reply(ctx, format!("Queued song: {}", song.metadata().title.clone().unwrap_or_default())).await?;
            } else {
                msg.reply(ctx, format!("Playing song: {}", song.metadata().title.clone().unwrap_or_default())).await?;
            }
        },
        Err(e) => {
            msg.reply(ctx, format!("Failed to play song. {}", e)).await?;
        }
    };

    Ok(())
}