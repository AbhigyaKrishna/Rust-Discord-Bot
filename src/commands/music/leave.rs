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
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = get_songbird_manager(ctx).await;

    if manager.get(guild_id).is_some() {
        let _ = manager.remove(guild_id).await;
        msg.channel_id.say(&ctx.http, "Left voice channel").await?;
    } else {
        msg.reply(ctx, "Not in a voice channel").await?;
    }

    Ok(())
}