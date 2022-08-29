use std::{collections::HashMap, ops::Index};

use super::config::Config;
use rand::{self, seq::SliceRandom};
use termion;
use colorful::{self, Color, core::{ColorInterface, color_string::CString}, Colorful};

pub const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub const LOWERCASE: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
pub const UPPERCASE: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
pub const SPECIAL: [char; 9] = ['!', '$', '%', '(', ')', '/', '#', '+', '?'];

#[derive(Debug, Clone)]
pub enum Chars{
    Uppercase,
    Lowercase,
    Digits,
    Special
}
impl Chars{
    pub fn chars(&self) -> Vec<char>{
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
    let mut chars = config.chars();
    remove_excl(&mut chars, config.exclude.clone());
    for _ in 0..config.len.clone(){
        let selected = chars.choose(&mut rand::thread_rng()).unwrap().to_owned().to_owned();
        out.push(selected);
    }

    out
}

pub fn calc_entropy(config: &Config) -> f64{
    f64::log2(f64::powf(config.chars().len() as f64, config.len as f64))
}

pub fn remove_excl<'a>(chars: &'a mut Vec<char>, exlcuded: Option<Vec<char>>) -> &'a Vec<char>{
    if exlcuded.is_none(){
        chars
    } else{
        // Some weird iterator stuff to remove the excluded chars
        let exlcuded = exlcuded.unwrap();
        for excl in exlcuded.iter(){
            chars.retain(|c| {c != excl})
        }
        chars
    }
}

pub fn format_output(password: String, entropy: f64) -> String{
    let colors = HashMap::from([
        ((0, 25), Color::Red),
        ((25, 60), Color::Orange1),
        ((60, 95), Color::Yellow),
        ((95, i128::MAX), Color::Green),
    ]);

    let mut e = CString::new(String::new());
    for ((min, max), color) in colors{
        if min <= (entropy as i128) && (entropy as i128) <= max{
            e = entropy.to_string().color(color);
            break
        }
    }

    format!("
{d}
{password}


Entropy: {e}
{d}
", d = "-".repeat(std::cmp::min(password.len(), termion::terminal_size().unwrap().0.into()))).to_string()
}