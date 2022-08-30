#![allow(unused)]

use inquire;
use super::config::Config;
use super::util::{Chars};

pub fn home() -> Config{
    let mut config = Config::default();
    let options = vec!["Set length", "Set characters", "Select preset", "Exclude characters", "Other options", "View config", "Done!"];
    'main: loop{
        let ans = inquire::Select::new("Generate Config!", options.clone()).with_help_message("Generate a config").prompt().unwrap();
        match ans{
            "Set length" => {config.len = set_length()},
            "Set characters" => {config.chars = set_chars()},
            "Select preset" => {config = select_preset()},
            "Exclude characters" => {config.exclude = select_exclude()},
            "Other options" => {other_options(&mut config);},
            "View config" => {print_conf(&config)},
            "Done!" => {print_conf(&config); break 'main},
            _ => {}
        }
    }
    config
}

fn set_length() -> i32{
    inquire::Text::new("Set length").prompt().unwrap().parse::<i32>().unwrap_or(Config::default().len)
}

fn other_options<'a>(config: &'a mut Config) -> &'a Config{
    let options = vec!["Check wordlists"];
    let anss = inquire::MultiSelect::new("Other options", options).prompt().unwrap();
    for ans in anss{
        match ans {
            "Check wordlists" => {config.check_wordlists = true}
            _ => {todo!()}
        }
    } 
    
    config
}

fn set_chars() -> Vec<Chars>{
    let mut out: Vec<Chars> = Vec::new();
    let options = vec!["Uppercase", "Lowercase", "Digits", "Special"];
    let anss = inquire::MultiSelect::new("Select which characters to include", options).prompt().unwrap();
    for ans in anss{
        out.push(match ans{
            "Uppercase" => Chars::Uppercase,
            "Lowercase" => Chars::Lowercase,
            "Digits" => Chars::Digits,
            "Special" => Chars::Digits,
            _ => todo!(),
        });
    }

    out
}

fn select_preset() -> Config{
    let options = vec!["Bad", "Simple", "Medium", "Advanced", "Secure", "Gynormous"];
    let ans = inquire::Select::new("Select preset!", options).prompt().unwrap();
    match ans{
        "Bad" => {Config::new(6, vec![Chars::Lowercase, Chars::Digits], None, false)},
        "Simple" => {Config::new(12, vec![Chars::Lowercase, Chars::Digits], None, false)},
        "Medium" => {Config::new(16, vec![Chars::Lowercase, Chars::Uppercase, Chars::Digits], None, false)},
        "Advanced" => {Config::new(18, vec![Chars::Lowercase, Chars::Uppercase, Chars::Digits], None, true)},
        "Secure" => {Config::new(24, vec![Chars::Lowercase, Chars::Uppercase, Chars::Digits, Chars::Special], None, true)},
        "Maximum security" => {Config::new(48, vec![Chars::Lowercase, Chars::Uppercase, Chars::Digits, Chars::Special], None, true)},
        "Gynormous" => {Config::new(1024, vec![Chars::Lowercase, Chars::Uppercase, Chars::Digits, Chars::Special], None, true)},
        _ => todo!(),
    }
}

fn select_exclude() -> Option<Vec<char>>{
    let ans = inquire::Text::new("Type characters to exclude (split by individual chars)").prompt().unwrap();
    if ans.len() == 0{return None}
    Some(Vec::from(ans.chars().collect::<Vec<char>>()))
}

fn print_conf(config: &Config){
    println!("{}", config);
}