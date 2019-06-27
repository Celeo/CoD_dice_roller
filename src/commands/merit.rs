use log::{debug, info};
use lazy_static::lazy_static;
use serenity::{
    client::Context,
    framework::standard::{CommandResult, macros::command},
    http::AttachmentType,
    model::channel::Message,
};
use std::path::Path;

lazy_static! {
    static ref MERIT_NAMES: Vec<&'static str> = vec![
        "Area of Expertise",
        "Common Sense",
        "Danger Sense",
        "Direction Sense",
        "Eidetic Memory",
        "Encyclopedic Knowledge",
        "Eye for the Strange",
        "Fast Reflexes",
        "Good Time Management",
        "Holistic Awareness",
        "Indomitable",
        "Interdisciplinary Specialty",
        "Investigative Aide",
        "Investigative Prodigy",
        "Language",
        "Library",
        "Meditative Mind",
        "Multilingual",
        "Patient",
        "Professional Training",
        "Tolerance for Biology",
        "Trained Observer",
        "Vice-Ridden",
        "Ambidextrous",
        "Automotive Genius",
        "Crack Driver",
        "Demolisher",
        "Double Jointed",
        "Fleet of Foot",
        "Giant",
        "Hardy",
        "Greyhound",
        "Iron Stamina",
        "Parkour",
        "Quick Draw",
        "Relentless",
        "Seizing the Edge",
        "Sleight of Hand",
        "Small-Framed",
        "Stunt Driver",
        "Allies",
        "Alternate Identity",
        "Anonymity",
        "Barfly",
        "Closed Book",
        "Contacts",
        "Fame",
        "Fast-Talking",
        "Fixer",
        "Hobbyist Clique",
        "Inspiring",
        "Iron Will",
        "Mentor",
        "Mystery Cult Initiation",
        "Pusher",
        "Resources",
        "Retainer",
        "Safe Place",
        "Small Unit Tactics",
        "Spin Doctor",
        "Staff",
        "Status",
        "Striking Looks",
        "Sympathetic",
        "Table Turner",
        "Takes One to Know One",
        "Taste",
        "True Friend",
        "Untouchable",
        "Aura Reading",
        "Automatic Writing",
        "Biokinesis",
        "Clairvoyance",
        "Curser",
        "Laying on Hands",
        "Medium",
        "Mind of a Madman",
        "Omen Sensitivity",
        "Numbing Touch",
        "Psychokinesis",
        "Psychometry",
        "Telekinesis",
        "Telepathy",
        "Thief of Fate",
        "Unseen Sense",
        "Armed Defense",
        "Cheap Shot",
        "Choke Hold",
        "Close Quarters Combat",
        "Defensive Combat",
        "Fighting Finesse",
        "Firefight",
        "Grappling",
        "Heavy Weapons",
        "Improvised Weaponry",
        "Iron Skin",
        "Light Weapons",
        "Marksmanship",
        "Martial Arts",
        "Police Tactics",
        "Shiv",
        "Street Fighting",
        "Unarmed Defense",
    ];
}

#[command]
pub fn merit(context: &mut Context, message: &Message) -> CommandResult {
    if !message.content.contains(' ') {
        info!("Merit command had no arguments");
        return Ok(());
    }
    let first_space_index = message.content.chars().position(|c| c == ' ').unwrap();
    let name_match = &message.content[(first_space_index + 1)..];
    debug!("Merit name match is: {}", &name_match);
    let name_stub = name_match.replace(" ", "_").to_lowercase();
    let file_name = format!("{}.png", name_stub);
    debug!("Looking up merit image: {}", &file_name);
    let file_path = Path::new("./merits").join(&file_name);
    if !file_path.exists() {
        message
            .channel_id
            .say(&context.http, "Could not find merit.")?;
        return Ok(());
    }
    message.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.title(&name_match);
            e.attachment(&file_name);
            e
        });
        m.add_file(AttachmentType::Path(&file_path))
    })?;
    Ok(())
}
