use serenity::model::prelude::*;
use serenity::framework::standard::Args;
use serenity::prelude::*;
use rand::Rng;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group
        }
    },
    prelude::*,
};

#[command]
pub async fn multiply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let one = args.single::<f64>().unwrap();
    let two = args.single::<f64>().unwrap();

    let product = one * two;

    let _ = msg.channel_id.say(ctx, product).await?;

    Ok(())
}

#[command]
pub async fn divide(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let one = args.single::<f64>().unwrap();
    let two = args.single::<f64>().unwrap();

    let quotient = one / two;

    let _ = msg.channel_id.say(ctx, quotient).await?;

    Ok(())
}
