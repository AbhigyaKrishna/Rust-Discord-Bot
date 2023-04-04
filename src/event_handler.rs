use serenity::{
    async_trait,
    model::prelude::Ready,
    prelude::{
        Context,
        EventHandler
    }
};

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _ctx: Context, data: Ready) {
        println!("{}#{} is connected!", data.user.name, data.user.discriminator);
    }

}