use crate::*;
use std::path::PathBuf;

pub fn run() {
    let config = Config::read_toml(PathBuf::from("copywriter.toml"))
        .unwrap();
}

