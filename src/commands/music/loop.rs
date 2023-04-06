use serenity::{
    framework::standard::{
        CommandResult,
        Args,
        macros::command
    },
    prelude::Context,
    model::prelude::Message,
};
use crate::music::get_songbird_manager;

#[command]
async fn r#loop(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    
    let manager = get_songbird_manager(ctx).await;
    if let Some(handler) = manager.get(guild_id) {
        let mut call = handler.lock().await;
    } else {
        msg.reply(ctx, "Not in a voice channel").await?;
    }
    
    Ok(())
}