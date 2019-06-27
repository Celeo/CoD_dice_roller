# cod_dice_roller

A Discord bot that rolls dice for Chronicles of Darkness.

Modeled off of [Dicecord-CoD](https://discordbots.org/bot/319289665347911680).

## Building

This bot is written in [Rust](https://www.rust-lang.org/).

1. Clone
1. `cargo build`

## Running

1. Create a new application with bot user on [Discord apps](https://discordapp.com/developers/applications/)
1. Create an `.env` file with the bot's secret as a `DISCORD_TOKEN` value.
1. Invite the bot to your guild with permissions for:
    1. Manage Emojis
    1. Read Text Channels & See Voice Channels
    1. Send Message
    1. Embed Links
    1. Attach Files
1. `cargo run`

### Merits

If you want the bot to support the `!merit <name>` command, then you need to create a `./merits` folder where the bot is running and populate it with image files that match the `<name>.png` file pattern.
