use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use crate::music::get_songbird_manager;

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|vc| vc.channel_id);

    if let Some(id) = channel_id {
        let manager = get_songbird_manager(ctx).await;
        manager.join(guild_id, id).await;
        msg.reply(ctx, "Joined channel").await?;
    } else {
        msg.reply(ctx, "Not in voice channel").await?;
    }

    Ok(())
}