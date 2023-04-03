use serenity::{
    framework::standard::{
        CommandResult,
        macros::command,
    },
    model::prelude::Message,
    prelude::Context,
};
use crate::music::get_songbird_manager;

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = get_songbird_manager(ctx).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();
        msg.channel_id.say(&ctx.http, "Skipped song").await?;
        if let Some(curr) = queue.current() {
            msg.channel_id.say(&ctx.http, format!("Now playing: {}", curr.metadata().title.clone().unwrap_or_default())).await?;
        }
    } else {
        msg.reply(ctx, "Not in a voice channel").await?;
    }

    Ok(())
}