# dice_roller_bot

A Discord bot that rolls dice for Chronicles of Darkness.

Modeled off of [Dicecord-CoD](https://discordbots.org/bot/319289665347911680).

## Building

This bot is written in [Rust](https://www.rust-lang.org/).

1. Clone
1. `cargo build`

## Running

1. Create a new application with bot user on [Discord apps](https://discordapp.com/developers/applications/)
1. Create an `.env` file with the bot's secret as a `DISCORD_TOKEN` value.
1. Invite the bot to your guild with the 'Send Message' permissions
1. `cargo run`
