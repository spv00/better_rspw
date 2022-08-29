mod lib;
use lib::config::Config;
use lib::util;
use lib::menu;

fn main() {
    //let config = Config::parse(std::env::args().collect()).unwrap_or_default();
    let config = menu::home();
    let pass = util::generate(&config);
    println!("{}", util::format_output(pass, util::calc_entropy(&config)))
}