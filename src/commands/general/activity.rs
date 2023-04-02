use convert_case::{Case, Casing};
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::prelude::{Activity, Message};
use serenity::prelude::Context;

#[command]
async fn activity(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() < 2 {
        msg.reply(ctx, "Invalid input!").await?;
        return Ok(());
    }

    let activity_type = args.single::<String>().unwrap().to_ascii_lowercase();
    let activity = args.rest();

    match activity_type.as_str() {
        "playing" => {
            ctx.set_activity(Activity::playing(activity.clone())).await;
        },
        "listening" => {
            ctx.set_activity(Activity::listening(activity.clone())).await;
        },
        "watching" => {
            ctx.set_activity(Activity::watching(activity.clone())).await;
        },
        "competing" => {
            ctx.set_activity(Activity::competing(activity.clone())).await;
        },
        _ => {
            msg.reply(ctx, "Invalid activity type! Valid option: playing, listening, watching, competing").await?;
            return Ok(())
        }
    };

    msg.reply(ctx, format!("Set activity to **{} {}**", activity_type.to_case(Case::Title), activity)).await?;
    Ok(())
}