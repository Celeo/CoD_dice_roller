use serenity::{
    client::Context,
    framework::standard::{Args, CommandResult, macros::command},
    model::channel::Message,
};

#[command]
pub fn health(_context: &mut Context, _message: &Message, _args: Args) -> CommandResult {
    // TODO
    Ok(())
}
