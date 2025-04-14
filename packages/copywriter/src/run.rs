use std::path::PathBuf;
use crate::*;

pub fn run() {
    let config = Config::read_toml(PathBuf::from("copywriter.toml"))
        .unwrap();
}
