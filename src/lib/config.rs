use super::util;
use util::Chars;
use inquire;

#[derive(Debug, Clone)]
pub struct Config{
    pub chars: Vec<util::Chars>,
    pub len: i32,
}

impl Config{
    pub fn new(len: i32, chars: Vec<util::Chars>) -> Config{
        Config { chars: chars, len: len }
    }

    pub fn parse(args: Vec<String>) -> Result<Config, &'static str>{
        //Parse an array of string primitives to a config. For example 16 -p uld wil result in a config of length 16 and with an uppercase, lowercase and digit pool
        //returns either the parsed Config or the default config
        let mut len = 12;
        let mut chars: Vec<Chars> = vec![Chars::Uppercase, Chars::Lowercase, Chars::Digits];
        if args.len() <= 0{
            return Ok(Config::default());
        }
        len = args.get(1).unwrap_or(&Config::default().len.to_string()).parse::<i32>().unwrap_or(Config::default().len);
        if args.contains(&"-p".to_string()){
            chars = Vec::new();
            let pool_i = args.iter().position(|x| {x == "-p"}).unwrap().to_owned();
            let pool = &args[pool_i + 1];
            if pool.contains(&"u".to_string()){chars.push(Chars::Uppercase)}
            if pool.contains(&"l".to_string()){chars.push(Chars::Lowercase)}
            if pool.contains(&"d".to_string()){chars.push(Chars::Digits)}
            if pool.contains(&"s".to_string()){chars.push(Chars::Digits)}
        }
        return Ok(Config::new(len, chars));
    }

    pub fn chars(&self) -> Vec<&str>{
        let mut out: Vec<&str> = vec![];
        for char in self.chars.iter(){
            for c in char.chars(){
                out.push(c)
            }
        };
        out
    }
}

impl Default for Config{
    fn default() -> Config{
        Config { chars: vec![Chars::Uppercase, Chars::Lowercase, Chars::Digits], len: 12 }
    }
}