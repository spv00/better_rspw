#![allow(unused)]

use std::{fmt::Debug, process::exit};
use super::{util, menu};
use util::Chars;

#[derive(Debug, Clone, PartialEq)]
pub struct Config{
    pub chars: Vec<util::Chars>,
    pub len: i32,
    pub exclude: Option<Vec<char>>,
    pub check_wordlists: bool,
}

impl Config{
    pub fn new(len: i32, chars: Vec<util::Chars>, exclude: Option<Vec<char>>, check_wordlists: bool) -> Config{
        Config { chars: chars, len: len, exclude: exclude, check_wordlists: check_wordlists}
    }

    pub fn parse(args: Vec<String>) -> Result<Config, ConfigError>{
        // Parse an array of string primitives to a config. For example 16 -p uld wil result in a config of length 16 and with an uppercase, lowercase and digit pool
        // returns either the parsed Config or the default config
        let mut len = Config::default().len;
        let mut chars: Vec<Chars> = vec![Chars::Uppercase, Chars::Lowercase, Chars::Digits];
        let mut excluded: Vec<char> = Vec::new();
        let mut check_wordlists = false;
        // Return if arg length is below 1
        if args.len() <= 0{
            return Ok(Self::default());
        }

        // Check for interactive
        if args.contains(&"-i".to_string()){
            return Ok(menu::home());
        }
        // Check for wordlists
        if args.contains(&"-w".to_string()){
            check_wordlists = true;
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

        return Ok(Config::new(len, chars, Some(excluded), check_wordlists));
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

impl std::fmt::Display for Config{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Length: {} || Characters: {:?} || Excluded: {:?} || Check Wordlists: {}", self.len, self.chars(), self.exclude, self.check_wordlists)
    }
}

impl Default for Config{
    fn default() -> Config{
        Config { chars: vec![Chars::Uppercase, Chars::Lowercase, Chars::Digits], len: 12, exclude: None, check_wordlists: false }
    }
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

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_parser(){
        assert_eq!(Config::parse(vec![]).unwrap(), Config::default());
        assert_eq!(Config::parse("-l 16 -p uld -w".split(" ").map(String::from).collect::<Vec<String>>()).unwrap(), Config::new(16, vec![Chars::Uppercase, Chars::Lowercase, Chars::Digits], Some(vec![]), true));
    }
}