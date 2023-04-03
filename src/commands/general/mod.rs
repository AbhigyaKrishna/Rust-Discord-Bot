mod activity;
mod ping;

use serenity::framework::standard::macros::group;
use crate::commands::general::{
    ping::PING_COMMAND,
    activity::ACTIVITY_COMMAND,
};

#[group]
#[commands(ping, activity)]
struct General;