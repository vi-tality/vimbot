use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Pong!");

    Ok(())
}

// really just an example of how shit works
#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "available commands: iwantemacs, iwantneovim, iwantvi, idontwantemacs, idontwantvi, idontwantneovim");
    Ok(())
}

#[command]
fn q(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut member = ctx
        .http
        .get_member(648963701734506497, *msg.author.id.as_u64())
        .unwrap();
    let _ = member.add_role(&ctx, RoleId(648972141169213440));
    msg.channel_id.say(&ctx.http, "I GNU it--you're verified.");
    Ok(())
}

// really just an example of how shit works
#[command]
fn iwantemacs(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut member = ctx
        .http
        .get_member(648963701734506497, *msg.author.id.as_u64())
        .unwrap();
    let _ = member.add_role(&ctx, RoleId(648997712351592459));
    msg.channel_id.say(&ctx.http, "added Emacs role");
    Ok(())
}

#[command]
fn iwantneovim(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut member = ctx
        .http
        .get_member(648963701734506497, *msg.author.id.as_u64())
        .unwrap();
    let _ = member.add_role(&ctx, RoleId(648997739119771689));
    msg.channel_id.say(&ctx.http, "added NeoVim role");
    Ok(())
}

#[command]
fn iwantvi(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut member = ctx
        .http
        .get_member(648963701734506497, *msg.author.id.as_u64())
        .unwrap();
    let _ = member.add_role(&ctx, RoleId(649275285253914654));
    msg.channel_id.say(&ctx.http, "added vi role");
    Ok(())
}

#[command]
fn idontwantemacs(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut member = ctx
        .http
        .get_member(648963701734506497, *msg.author.id.as_u64())
        .unwrap();
    let _ = member.remove_role(&ctx, RoleId(648997712351592459));
    msg.channel_id.say(&ctx.http, "removed Emacs role");
    Ok(())
}

#[command]
fn idontwantneovim(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut member = ctx
        .http
        .get_member(648963701734506497, *msg.author.id.as_u64())
        .unwrap();
    let _ = member.remove_role(&ctx, RoleId(648997739119771689));
    msg.channel_id.say(&ctx.http, "removed NeoVim role");
    Ok(())
}

#[command]
fn idontwantvi(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut member = ctx
        .http
        .get_member(648963701734506497, *msg.author.id.as_u64())
        .unwrap();
    let _ = member.remove_role(&ctx.http, RoleId(649275285253914654));
    msg.channel_id.say(&ctx.http, "removed vi role");
    Ok(())
}
