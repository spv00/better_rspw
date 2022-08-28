use super::config::Config;
use rand::{self, seq::SliceRandom};

pub const DIGITS: [&'static str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
pub const LOWERCASE: [&'static str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
pub const UPPERCASE: [&'static str; 26] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
pub const SPECIAL: [&'static str; 9] = ["!", "$", "%", "(", ")", "/", "#", "+", "?"];

#[derive(Debug, Clone)]
pub enum Chars{
    Uppercase,
    Lowercase,
    Digits,
    Special
}
impl Chars{
    pub fn chars(&self) -> Vec<&str>{
        match *self {
            Chars::Uppercase => UPPERCASE.to_vec(),
            Chars::Lowercase => LOWERCASE.to_vec(),
            Chars::Digits => DIGITS.to_vec(),
            Chars::Special => SPECIAL.to_vec(),
        }
    }
}

pub fn generate(config: &Config) -> String{
    let mut out: String = String::new();
    for _ in 0..config.len.clone(){
        out.push(
            config.chars().choose(&mut rand::thread_rng()).unwrap().to_owned().to_owned().chars().next().expect("Something went wrong")
        );
    }

    out
}