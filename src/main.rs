use log::{debug, error, info, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{
        Appender,
        Config,
        Logger,
        Root,
    },
    encode::pattern::PatternEncoder,
};
use serenity::{
    client::{Client, Context},
    framework::{StandardFramework, standard::macros::group},
    prelude::EventHandler,
    model::gateway::Ready,
};
use std::{env, path::Path};

mod commands;
use commands::{character::*, help::*, merit::*, roll::*};

mod util;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _context: Context, _ready: Ready) {
        info!("Bot connected");
    }
}

group!({
    name: "general",
    options: {},
    commands: [character, help, merit, roll]
});

fn setup_logger() {
    if Path::new("./log4rs.yml").exists() {
        println!("Loading logging config from file");
        log4rs::init_file("log4rs.yml", Default::default())
            .expect("Could not load/apply log4rs configuration");
    } else {
        println!("Loading coded logging config");
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} {h({l})} {m}{n}")))
            .build();
        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .additive(false)
                    .build("dice_roller_bot", LevelFilter::Debug),
            )
            .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
            .unwrap();
        log4rs::init_config(config).expect("Could not set log4rs configuraton");
    }
}

fn main() {
    setup_logger();
    debug!("Starting up");

    info!("Loading environment");
    kankyo::load().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN env var");

    info!("Creating client");
    let mut client = Client::new(&token, Handler).expect("Could not create client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!").case_insensitivity(true))
            .group(&GENERAL_GROUP)
            .on_dispatch_error(|_context, message, error| {
                error!(
                    "Command error occurred in '{}': {:?}",
                    message.content, error
                );
            }),
    );

    info!("Starting client");
    if let Err(err) = client.start() {
        error!("Could not start client: {}", err);
    }
}
