use serenity::framework::standard::macros::group;
use crate::commands::music::join::JOIN_COMMAND;
use crate::commands::music::leave::LEAVE_COMMAND;
use crate::commands::music::play::PLAY_COMMAND;
use crate::commands::music::stop::STOP_COMMAND;
use crate::commands::music::skip::SKIP_COMMAND;

mod join;
mod leave;
mod play;
mod stop;
mod skip;

#[group]
#[commands(join, leave, play, stop, skip)]
struct Music;