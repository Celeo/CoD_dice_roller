use lazy_static::lazy_static;
use log::debug;
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use regex::Regex;
use serenity::{
    client::Context,
    framework::standard::{Args, CommandResult, macros::command},
    model::channel::Message,
    utils::MessageBuilder,
};
use std::{collections::HashMap, fmt};
use crate::util::{
    constants::LOAD_PATH,
    characters::{Character, CharacterStore},
};

const CHANCE: &str = "chance";

lazy_static! {
    static ref REGEX_NUMERIC: Regex = Regex::new(r#"^\d+$"#).unwrap();
    static ref REGEX_WHITESPACE: Regex = Regex::new(r#"\W{2,}"#).unwrap();
    static ref REGEX_AGAIN: Regex = Regex::new(r#"^(?:no)?\d+again$"#).unwrap();
}

/// The types of modifiers that can be applied to a roll.
///
/// Determines which values are re-rolled, increasing the
/// changes of getting successes.
#[derive(Debug, PartialEq)]
enum RollModifier {
    Again10,
    Again9,
    Again8,
    NoAgain,
}

/// Returns the `RollModifier` for the string.
///
/// # Arguments
///
/// * `s` - the string
///
/// # Examples
///
/// ```rust
/// let roll_mod = mod_for_str("9again");
/// ```
fn mod_for_str(s: &str) -> RollModifier {
    if s.contains("no10again") {
        RollModifier::NoAgain
    } else if s.contains("9again") {
        RollModifier::Again9
    } else if s.contains("8again") {
        RollModifier::Again8
    } else {
        RollModifier::Again10
    }
}

/// Returns whether the value and modifier constitute a re-roll.
///
/// # Arguments
///
/// * `val` - die roll value
/// * `modifier` - modifier
///
/// # Examples
///
/// ```rust
/// let should = roll_again(9, &RollModifier::Again10);
/// ```
fn roll_again(val: u64, modifier: &RollModifier) -> bool {
    modifier == &RollModifier::Again10 && val == 10
        || modifier == &RollModifier::Again9 && val >= 9
        || modifier == &RollModifier::Again8 && val >= 8
}

/// Result of rolling a die.
#[derive(Debug)]
struct Roll {
    val: u64,
    is_bonus: bool,
}

impl fmt::Display for Roll {
    /// Display impl.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_bonus {
            write!(f, "({})", self.val)
        } else {
            write!(f, "{}", self.val)
        }
    }
}

/// Roll dice.
///
/// # Arguments
///
/// * `dice` - string to roll
/// * `modifier` - roll modifier
///
/// # Examples
///
/// ```rust
/// let result = roll("5", &RollModifier::Again9);
/// ```
fn roll_dice(dice: &str, modifier: &RollModifier) -> Vec<Roll> {
    let between = Uniform::new_inclusive(1, 10);
    let mut rng = thread_rng();

    if dice == CHANCE {
        let val = between.sample(&mut rng);
        vec![Roll {
            val,
            is_bonus: false,
        }]
    } else {
        let mut rolls = vec![];
        for _ in 1..=dice.parse::<u64>().unwrap() {
            let mut first = true;
            loop {
                let next_val = between.sample(&mut rng);
                rolls.push(Roll {
                    val: next_val,
                    is_bonus: !first,
                });
                if !roll_again(next_val, modifier) {
                    break;
                }
                first = false;
            }
        }
        rolls
    }
}

#[derive(Debug)]
struct AttribRollResult {
    pool: i64,
    modifier: RollModifier,
    attributes: HashMap<String, i64>,
    attribs_not_found: Vec<String>,
}

fn roll_attribs(character: &Character, line: &str) -> AttribRollResult {
    let mut attributes = HashMap::new();
    let mut attribs_not_found = vec![];
    let again_parts: Vec<&str> = line
        .split_whitespace()
        .filter(|p| REGEX_AGAIN.is_match(p))
        .collect();
    let (line, modifier) = if again_parts.is_empty() {
        (line.to_owned(), "10again")
    } else {
        (line.replace(again_parts[0], ""), again_parts[0].trim())
    };
    let line = line.replace("+", " + ").replace("-", " - ");

    let mut pool = 0i64;
    let mut multiplier = 1i8;
    for part in line.split_whitespace() {
        let part = part.trim();
        if part == "-" {
            multiplier = -1;
            continue;
        }
        if REGEX_NUMERIC.is_match(&part) {
            pool += part.parse::<i64>().unwrap() * i64::from(multiplier);
        } else if part != "+" {
            let (found, val) = character.get_value(part);
            if !found {
                attribs_not_found.push(part.to_owned());
            } else {
                attributes.insert(part.to_owned(), val);
            }
            pool += val * i64::from(multiplier);
        }
        multiplier = 1;
    }
    AttribRollResult {
        pool,
        modifier: mod_for_str(modifier),
        attributes,
        attribs_not_found,
    }
}

/// Return text containing the number of successes.
///
/// # Arguments
///
/// * `rolls` - rolls
///
/// # Examples
///
/// ```rust
/// let sc = count_successes(&rolls);
/// ```
fn count_successes(rolls: &[Roll]) -> String {
    let count = rolls.iter().filter(|e| e.val > 7).count();
    let text = if count != 1 { "successes" } else { "success" };
    format!("{} {}: ", count, text)
}

#[command]
pub fn roll(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        debug!("No args supplied to roll command");
        return Ok(());
    }
    let dice = args.parse::<String>().unwrap();
    if dice == CHANCE || REGEX_NUMERIC.is_match(&dice) {
        let result = roll_dice(&dice, &mod_for_str(&message.content));
        let response = if dice == CHANCE {
            if result[0].val == 10 {
                MessageBuilder::new()
                    .mention(&message.author)
                    .push(" rolled a chance die and succeeded!")
                    .build()
            } else {
                MessageBuilder::new()
                    .mention(&message.author)
                    .push(" rolled a chance die and failed: ")
                    .push(result[0].val)
                    .build()
            }
        } else {
            MessageBuilder::new()
                .mention(&message.author)
                .push(" rolled ")
                .push(dice)
                .push(" dice and got ")
                .push(count_successes(&result))
                .push(
                    result
                        .iter()
                        .map(|e| format!("{}", e))
                        .collect::<Vec<_>>()
                        .join(", "),
                )
                .build()
        };
        message.channel_id.say(&context.http, &response)?;
    } else {
        let cs = CharacterStore::from_file(&LOAD_PATH).unwrap();
        let new_character = Character::new(&message.author.name);
        let character = match cs.get(&message.author.name) {
            Some(c) => c,
            None => &new_character,
        };
        let attrib_result = roll_attribs(&character, &message.content.trim().replace("!roll ", ""));
        let roll_result = roll_dice(&attrib_result.pool.to_string(), &attrib_result.modifier);
        let mut builder = MessageBuilder::new()
            .mention(&message.author)
            .push(" rolled ")
            .push(attrib_result.pool)
            .push(" dice [")
            .push(
                attrib_result
                    .attributes
                    .iter()
                    .map(|(k, v)| format!("{} = {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", "),
            )
            .push("] and got ")
            .push(count_successes(&roll_result))
            .push(
                roll_result
                    .iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<_>>()
                    .join(", "),
            )
            .clone();
        if !attrib_result.attribs_not_found.is_empty() {
            builder
                .push("\n\nWarning: these attributes were not found and defaulted to 0: ")
                .push(
                    attrib_result
                        .attribs_not_found
                        .iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                );
        }
        message.channel_id.say(&context.http, &builder.build())?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{count_successes, mod_for_str, Roll, roll_again, roll_attribs, RollModifier};
    use crate::util::characters::Character;

    #[test]
    fn test_mod_for_str() {
        assert_eq!(mod_for_str(""), RollModifier::Again10);
        assert_eq!(mod_for_str("9again"), RollModifier::Again9);
        assert_eq!(mod_for_str("8again"), RollModifier::Again8);
        assert_eq!(mod_for_str("no10again"), RollModifier::NoAgain);
    }

    #[test]
    fn test_roll_again() {
        assert!(!roll_again(10, &RollModifier::NoAgain));

        assert!(roll_again(10, &RollModifier::Again10));
        assert!(!roll_again(9, &RollModifier::Again10));
        assert!(!roll_again(8, &RollModifier::Again10));
        assert!(!roll_again(7, &RollModifier::Again10));

        assert!(roll_again(10, &RollModifier::Again9));
        assert!(roll_again(9, &RollModifier::Again9));
        assert!(!roll_again(8, &RollModifier::Again9));
        assert!(!roll_again(7, &RollModifier::Again8));

        assert!(roll_again(10, &RollModifier::Again8));
        assert!(roll_again(9, &RollModifier::Again8));
        assert!(roll_again(8, &RollModifier::Again8));
        assert!(!roll_again(7, &RollModifier::Again8));
    }

    #[test]
    fn test_count_successes() {
        let cs = count_successes(&[Roll {
            val: 1,
            is_bonus: false,
        }]);

        assert_eq!(cs, "0 successes: ");

        let cs = count_successes(&[Roll {
            val: 10,
            is_bonus: false,
        }]);

        assert_eq!(cs, "1 success: ");

        let cs = count_successes(&[
            Roll {
                val: 10,
                is_bonus: false,
            },
            Roll {
                val: 8,
                is_bonus: false,
            },
        ]);

        assert_eq!(cs, "2 successes: ");
    }

    #[test]
    fn test_roll_attribs() {
        let s = "  strength +  athletics- 1 9again";
        let mut c = Character::new("");
        let res = roll_attribs(&c, &s);

        assert_eq!(res.pool, -1);
        assert_eq!(res.modifier, RollModifier::Again9);
        assert_eq!(res.attribs_not_found, vec!["strength", "athletics"]);

        c.set_value("strength", 3);
        c.set_value("athletics", 1);
        let res = roll_attribs(&c, &s);

        assert_eq!(res.pool, 3);
        assert_eq!(res.modifier, RollModifier::Again9);
        assert!(res.attribs_not_found.is_empty());
    }
}
