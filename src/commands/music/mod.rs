mod join;
mod leave;
mod play;
mod stop;
mod skip;
mod r#loop;

use serenity::framework::standard::macros::group;
use crate::commands::music::{
    join::JOIN_COMMAND,
    leave::LEAVE_COMMAND,
    play::PLAY_COMMAND,
    stop::STOP_COMMAND,
    skip::SKIP_COMMAND,
    r#loop::LOOP_COMMAND,
};

#[group]
#[commands(join, leave, play, stop, skip, loop)]
struct Music;