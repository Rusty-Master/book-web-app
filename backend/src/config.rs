use std::{collections::HashMap, env, fs};

pub struct Config {
    pub map: HashMap<String, serde_yaml::Value>,
}

impl Config {
    pub fn new() -> Config {
        let args: Vec<String> = env::args().collect();
        let file_path = &args[args.len() - 1];

        let file = fs::File::open(file_path).unwrap();
        let map = serde_yaml::from_reader(file).unwrap();

        Config { map }
    }
}
