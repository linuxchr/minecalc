use std::env;

pub struct Config {
    pub cli: bool,
}

impl Config {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().skip(1).collect();
        let cli = args.contains(&String::from("--cli"));
        Config { cli }
    }
}
