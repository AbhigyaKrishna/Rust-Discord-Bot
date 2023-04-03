mod join;
mod leave;
mod play;
mod stop;
mod skip;

use serenity::framework::standard::macros::group;
use crate::commands::music::{
    join::JOIN_COMMAND,
    leave::LEAVE_COMMAND,
    play::PLAY_COMMAND,
    stop::STOP_COMMAND,
    skip::SKIP_COMMAND,
};

#[group]
#[commands(join, leave, play, stop, skip)]
struct Music;