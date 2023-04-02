use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::Context;

#[command]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|vc| vc.channel_id);

    if let Some(id) = channel_id {
        let manager = songbird::get(ctx).await
            .expect("Songbird Voice client placed in at initialisation.").clone();
        manager.join(guild_id, id).await;
    } else {
        msg.reply(ctx, "Not in voice channel").await?;
    }

    Ok(())
}