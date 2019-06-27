use log::debug;
use serenity::{
    client::Context,
    framework::standard::{Args, CommandResult, macros::command},
    model::channel::Message,
    utils::MessageBuilder,
};
use crate::util::{constants::LOAD_PATH, characters::CharacterStore};

#[command]
pub fn character(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let mut args = args;
    if args.is_empty() {
        debug!("No args supplied to character command");
        return Ok(());
    }
    let first_arg = args.single::<String>().unwrap();
    let username = &message.author.name;
    let mut cs = CharacterStore::from_file(&LOAD_PATH).unwrap();
    let character = cs.get_mut(username);
    if first_arg == "print" {
        let response = MessageBuilder::new()
            .push_codeblock(&character, None)
            .build();
        message.channel_id.say(&context.http, &response)?;
    } else if first_arg == "edit" {
        if args.len() != 3 {
            message
                .channel_id
                .say(&context.http, "`!character edit <stat_name> <stat_value>`")?;
            return Ok(());
        }
        let stat_key = args.single::<String>().unwrap();
        let stat_value = match args.single::<i64>() {
            Ok(i) => i,
            Err(_) => {
                message.channel_id.say(
                    &context.http,
                    "`The <stat_value> argument must be a number`",
                )?;
                return Ok(());
            }
        };
        character.set_value(&stat_key, stat_value);
        message.react(&context, "👍")?;
    }
    cs.save(&LOAD_PATH).unwrap();
    Ok(())
}
