#[macro_use]
extern crate clap;
use clap::App;

use std::io::{self, Read};

pub mod lib;

pub fn main() {
    // read CLI config
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut buffer = String::new();
    let mut stdin = io::stdin();

    // read stdin
    match stdin.read_to_string(&mut buffer) {
        Ok(_) => {
            if matches.is_present("compress") {
                print!("{}", lib::compress(&buffer));
            } else if matches.is_present("decompress") {
                print!("{}", lib::decompress(&buffer));
            }
        }
        Err(why) => panic!("{}", why),
    }
}
