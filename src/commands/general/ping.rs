use serenity::{
    framework::standard::{
        CommandResult,
        macros::command
    },
    model::prelude::Message,
    prelude::Context,
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}