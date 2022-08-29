use std::{fmt::Debug, process::exit};

use super::util;
use util::Chars;

#[derive(Debug, Clone)]
pub struct Config{
    pub chars: Vec<util::Chars>,
    pub len: i32,
    pub exclude: Option<Vec<char>>,
}

#[derive(Debug, Clone)]
pub struct ConfigError;
impl std::error::Error for ConfigError{
    fn description(&self) -> &str {
        "Something went wrong parsing your command line arguments."
    }
}
impl std::fmt::Display for ConfigError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse config! Exiting!")
    }
}

impl Config{
    pub fn new(len: i32, chars: Vec<util::Chars>, exclude: Option<Vec<char>>) -> Config{
        Config { chars: chars, len: len, exclude: exclude}
    }

    pub fn parse(args: Vec<String>) -> Result<Config, ConfigError>{
        // Parse an array of string primitives to a config. For example 16 -p uld wil result in a config of length 16 and with an uppercase, lowercase and digit pool
        // returns either the parsed Config or the default config
        let mut len = 12;
        let mut chars: Vec<Chars> = vec![Chars::Uppercase, Chars::Lowercase, Chars::Digits];
        let mut excluded: Vec<char> = Vec::new();
        // Return if arg length is below 1 
        if args.len() <= 0{
            return Ok(Config::default());
        }
        // Set length by first argument
        len = args.get(1).unwrap_or(&Config::default().len.to_string()).parse::<i32>().unwrap_or(Config::default().len);
        // Get pool values by -p ulds(uppercase, lowercase, digits, special)
        if args.contains(&"-p".to_string()){
            chars = Vec::new();
            let pool_i = args.iter().position(|x| {x == "-p"}).unwrap().to_owned();
            let pool = match args.get(pool_i + 1).ok_or(ConfigError){
                Ok(v) => v,
                Err(e) => {eprintln!("{}", e); exit(1)}
            };
            if pool.contains(&"u".to_string()){chars.push(Chars::Uppercase)}
            if pool.contains(&"l".to_string()){chars.push(Chars::Lowercase)}
            if pool.contains(&"d".to_string()){chars.push(Chars::Digits)}
            if pool.contains(&"s".to_string()){chars.push(Chars::Digits)}
        }
        // Get excluded
        if args.contains(&"-e".to_string()){
            let excl_i = args.iter().position(|x| {x == "-e"}).unwrap().to_owned();
            let excl = match args.get(excl_i + 1).ok_or(ConfigError){
                Ok(v) => v,
                Err(e) => {eprintln!("{}", e); exit(1)}
            };
            for c in excl.chars(){
                excluded.push(c);
            };
        };
        return Ok(Config::new(len, chars, Some(excluded)));
    }

    pub fn chars(&self) -> Vec<char>{
        let mut out: Vec<char> = vec![];
        for char in self.chars.iter(){
            for c in char.clone().chars(){
                out.push(c)
            }
        };
        out
    }
}

impl Default for Config{
    fn default() -> Config{
        Config { chars: vec![Chars::Uppercase, Chars::Lowercase, Chars::Digits], len: 12, exclude: None }
    }
}