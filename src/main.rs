#![allow(unused)]

mod lib;
use lib::config::Config;
use lib::menu;
use lib::util;
mod gui;
fn main() {
    //let config = Config::parse(std::env::args().collect()).unwrap_or_default();
    //let pass = util::generate(&config);
    //println!("{}", util::format_output(pass, util::calc_entropy(&config), &config));
    gui::main::main();
}
