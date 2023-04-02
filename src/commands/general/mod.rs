use serenity::framework::standard::macros::group;
use crate::commands::general::ping::PING_COMMAND;
use crate::commands::general::activity::ACTIVITY_COMMAND;

mod activity;
mod ping;

#[group]
#[commands(ping, activity)]
struct General;