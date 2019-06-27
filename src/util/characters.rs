use failure::Error;
use prettytable::{cell, format, row, Table};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt,
    fs,
    path::Path,
};

/// Represents a character's health.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Health {
    max: u64,
    bashing: u64,
    lethal: u64,
    aggravated: u64,
}

impl Health {
    /// Construct new health tracker.
    fn new() -> Self {
        Health {
            max: 0,
            bashing: 0,
            lethal: 0,
            aggravated: 0,
        }
    }
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.max == 0 {
            return writeln!(f, "No health info");
        }
        writeln!(f, "Health (max {}):", self.max)?;
        let mut boxes = vec![];
        for _ in 0..self.aggravated {
            boxes.push("A");
        }
        for _ in 0..self.lethal {
            boxes.push("L");
        }
        for _ in 0..self.bashing {
            boxes.push("B");
        }
        for _ in 0..(self.max - self.aggravated - self.lethal - self.bashing) {
            boxes.push(" ");
        }
        let mut table = Table::new();
        table.add_row(boxes.iter().map(|b| cell!(b)).collect());
        write!(f, "{}", table)?;
        Ok(())
    }
}

/// Represents a single player character.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Character {
    name: String,
    stats: HashMap<String, i64>,
    health: Health,
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.stats.is_empty() {
            return write!(f, "{}\n\nNo stats info", self.health);
        }
        let mut table = Table::new();
        table.set_titles(row!["Name", "Value", "", "Name", "Value"]);
        let size = self.stats.len();
        let half_rounded = f64::ceil(self.stats.len() as f64 / 2f64) as u64;
        let items: Vec<_> = self.stats.iter().map(|(k, &v)| (k, v)).collect();
        for index in 0..half_rounded {
            let index = index as usize;
            let index_upper = index + half_rounded as usize;
            if index_upper >= size {
                table.add_row(row![items[index].0, items[index].1, "", "", "",]);
            } else {
                table.add_row(row![
                    items[index].0,
                    &format!("  {}", items[index].1),
                    "",
                    items[index_upper].0,
                    &format!("  {}", items[index_upper].1),
                ]);
            }
        }
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        write!(f, "{}\nStats:\n{}", self.health, table)
    }
}

impl Character {
    /// Create a new struct.
    ///
    /// # Arguments
    ///
    /// * `name` - name of the character
    ///
    /// # Examples
    ///
    /// ```rust
    /// let character = Character::new("Paul");
    /// ```
    pub fn new(name: &str) -> Self {
        Character {
            name: name.to_owned(),
            stats: HashMap::new(),
            health: Health::new(),
        }
    }

    /// Attempt to get a stored value.
    ///
    /// If the value is not found in the store, 0 is returned.
    /// The boolean argument represents whether or not the
    /// value was returned. Useful in determining whether
    /// or not the value was not found, or actually stored as 0.
    ///
    /// # Arguments
    ///
    /// * `key` - which key to fetch
    ///
    /// # Examples
    ///
    /// ```rust
    /// let (found, value) = character.get_value("foo");
    /// ```
    pub fn get_value(&self, key: &str) -> (bool, i64) {
        let key = key.to_lowercase();
        match self.stats.get(&key) {
            Some(i) => (true, *i),
            None => (false, 0),
        }
    }

    /// Sets the value by name.
    ///
    /// # Arguments
    ///
    /// * `key` - stats name to set
    /// * `value`- stat value
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut character = Character::new("Paul");
    /// character.set_value("something", 100);
    /// ```
    pub fn set_value(&mut self, key: &str, value: i64) {
        self.stats.insert(key.to_lowercase().to_owned(), value);
    }
}

/// Collections of characters.
#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterStore {
    characters: Vec<Character>,
}

impl CharacterStore {
    /// Get a stored character by name.
    ///
    /// Returns an immutable reference, only usable for reading.
    /// If no stored character by that name is found, None is returned.
    ///
    /// # Arguments
    ///
    /// * `name` - name of the character to retrieve
    ///
    /// # Examples
    ///
    /// ```rust
    /// let character = match character_store.get("Paul") {
    ///     Some(c) => c,
    ///     None => panic!("No character found"),
    /// };
    /// ```
    pub fn get(&self, name: &str) -> Option<&Character> {
        for character in &self.characters {
            if character.name == name {
                return Some(character);
            }
        }
        None
    }

    /// Get a stored character by name.
    ///
    /// Returns a mutable reference suitable for updating stats.
    /// If there is no character by that name found, a new one
    /// is created.
    ///
    /// # Arguments
    ///
    /// * `name` - name of the character to retrieve
    ///
    /// # Examples
    ///
    /// ```rust
    /// let character = match character_store.get_mut("Paul");
    /// ```
    pub fn get_mut(&mut self, name: &str) -> &mut Character {
        match self.characters.iter().position(|c| c.name == name) {
            Some(i) => self.characters.get_mut(i).unwrap(),
            None => {
                let c = Character::new(&name);
                self.characters.push(c);
                self.characters.last_mut().unwrap()
            }
        }
    }

    /// Loads the store from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - path to the file
    ///
    /// # Examples
    ///
    /// ```rust
    /// let store = CharacterStore::from_file(std::path::Path::new("./data.json")).unwrap();
    /// ```
    pub fn from_file(path: &Path) -> Result<Self, Error> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => String::from(r#"{"characters":[]}"#),
        };
        let cs = serde_json::from_str(&content)?;
        Ok(cs)
    }

    /// Save the store to a JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - path to output file
    ///
    /// # Examples
    ///
    /// ```
    /// character_store.save(std::path::Path::new("./data.json")).unwrap();
    /// ```
    pub fn save(&self, path: &Path) -> Result<(), Error> {
        let output = serde_json::to_string(&self)?;
        fs::write(&path, &output)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{Character, CharacterStore};
    use std::fs;
    use tempdir::TempDir;

    #[test]
    fn test_store_from_file() {
        let temp = TempDir::new("dicebot").unwrap();
        let json_data = r#"{
            "characters": [
                {
                    "name": "Paul Roberts",
                    "stats": {
                        "intelligence": 1,
                        "wits": 3,
                        "resolve": 3,
                        "strength": 3,
                        "dexterity": 3,
                        "stamina": 2,
                        "presence": 2,
                        "manipulation": 1,
                        "composure": 3,
                        "academics": 0
                    }
                }
            ]
        }"#;
        let data_file = temp.path().join("data.json");
        fs::write(&data_file, json_data).unwrap();
        let cs = CharacterStore::from_file(&data_file).unwrap();

        assert_eq!(cs.characters.len(), 1);
        assert_eq!(cs.characters[0].stats.len(), 10);
        assert_eq!(cs.get("Paul Roberts").unwrap().get_value("wits"), (true, 3));
    }

    #[test]
    fn test_store_save() {
        let temp = TempDir::new("dicebot").unwrap();
        let mut ch = Character::new("A");
        ch.set_value("a", 100);
        let cs = CharacterStore {
            characters: vec![ch],
        };
        let output_path = temp.path().join("output.json");
        cs.save(output_path.as_path()).unwrap();

        let read_back = fs::read_to_string(output_path.as_path()).unwrap();
        let expected = r#"{"characters":[{"name":"A","stats":{"a":100}}]}"#;
        assert_eq!(read_back, expected);
    }

    #[test]
    fn test_character_get_set() {
        let mut c = Character::new("A");

        assert_eq!(c.name, "A");
        assert!(c.stats.is_empty());

        c.set_value("a", 5);
        c.set_value("a", 10);
        c.set_value("b", 5);

        assert_eq!(c.stats.len(), 2);
        assert_eq!(c.get_value("a"), (true, 10));
        assert_eq!(c.get_value("b"), (true, 5));
        assert_eq!(c.get_value("c"), (false, 0));
    }

    #[test]
    fn test_get_mut() {
        let mut cs = CharacterStore { characters: vec![] };
        let c = cs.get_mut("Paul");

        assert_eq!(c.get_value("foo"), (false, 0));

        c.set_value("foo", 1);

        assert_eq!(c.get_value("foo"), (true, 1));
    }
}
