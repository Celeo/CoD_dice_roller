use serenity::{
    client::Context,
    framework::standard::{CommandResult, macros::command},
    model::channel::Message,
    utils::MessageBuilder,
};

const HELP: &str = "Chronicles of Darkness dice roller bot

To use, type '!roll # <mod>', where # is a positive number or 'chance', and <what> is one of:

* 9again - to re-roll 10s and 9s
* 8again - to re-roll 10s, 9s, and 8s
* no10again - to not re-roll any values

Note that the '<what>' portion is optional.

Examples:

* !roll 4
* !roll chance
* !roll 10 9again

You can also edit a character reference with the following commands:

* !stats print|show
* !stats edit <name> <value>

Then, you can roll using those references, like:

!character edit strength 3
!roll strength + 1 9again
";

#[command]
pub fn help(context: &mut Context, message: &Message) -> CommandResult {
    let response = MessageBuilder::new().push_codeblock(HELP, None).build();
    message.channel_id.say(&context.http, &response)?;
    Ok(())
}
