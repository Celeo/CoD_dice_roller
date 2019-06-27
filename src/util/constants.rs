use lazy_static::lazy_static;
use std::path::Path;

#[cfg(not(test))]
lazy_static! {
    pub static ref LOAD_PATH: &'static Path = Path::new("./data.json");
}

#[cfg(test)]
lazy_static! {
    pub static ref LOAD_PATH: &'static Path = Path::new("./test-data.json");
}
