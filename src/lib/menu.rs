use inquire;
use super::config::Config;
use super::util::{Chars, self};

pub fn home() -> Config{
    let mut config = Config::default();
    let mut options = vec!["Set length", "Set characters", "Select preset", "View config", "Done!"];
    'main: loop{
        let ans = inquire::Select::new("Generate Config!", options.clone()).with_help_message("Generate a config").prompt().unwrap();
        match ans{
            "Set length" => {config.len = set_length()},
            "Set characters" => {config.chars = set_chars()},
            "Select preset" => {config = select_preset()},
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
    let options = vec!["Default", "Simple", "Medium", "Advanced", "Secure", "Maximum security"];
    let ans = inquire::Select::new("Select preset!", options).prompt().unwrap();
    match ans{
        "Default" => {Config::new(6, vec![Chars::Lowercase, Chars::Digits])},
        "Simple" => {Config::new(12, vec![Chars::Lowercase, Chars::Digits])},
        "Medium" => {Config::new(16, vec![Chars::Lowercase, Chars::Uppercase, Chars::Digits])},
        "Advanced" => {Config::new(18, vec![Chars::Lowercase, Chars::Uppercase, Chars::Digits])},
        "Secure" => {Config::new(24, vec![Chars::Lowercase, Chars::Uppercase, Chars::Digits, Chars::Special])},
        "Maximum security" => {Config::new(48, vec![Chars::Lowercase, Chars::Uppercase, Chars::Digits, Chars::Special])},
        _ => todo!(),
    }
}

fn print_conf(config: &Config){
    println!("{:?}", config);
}