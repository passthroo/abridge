use std::char;
use std::io::{self, Read};
use std::mem;

use substring::Substring;

#[macro_use]
extern crate clap;
use clap::App;

#[macro_use]
extern crate more_asserts;

pub fn compress(buf: &str) -> String {
    let mut line_prev = String::new();
    let mut result = String::new();

    for line in buf.lines() {
        let both = line.chars().zip(line_prev.chars());
        let len = line.chars().count();
        let mut matches = 0;

        for chars in both {
            if chars.0 == chars.1 {
                matches += 1;
            } else {
                break;
            }
        }

        let prefix = (matches + 1) as u8 as char;
        let compressed = line.substring(matches, len);

        result.push(prefix);
        result.push_str(compressed);

        mem::swap(&mut line.to_string(), &mut line_prev);
    }

    result
}

pub fn decompress(buf: &str) -> String {
    let mut chars = buf.chars().peekable();
    let mut result = String::new();
    let mut word = String::new();

    let mut i = chars.next().unwrap() as i32;
    assert_eq!(i, 1);

    let mut last_char = 0;
    let mut last_max = 0;

    while chars.peek().is_some() {
        if i == 0 {
            i = chars.next().unwrap() as i32;
        }

        i -= 1;
        assert_ge!(i, 0);
        assert_le!(i, last_max);

        for c in &mut chars {
            last_char = c as i32;
            if last_char <= 32 || i > 255 {
                break;
            }
            word.push(c);
            i += 1;
        }

        assert_le!(i, 255);
        last_max = i;

        word.push('\n');
        result.push_str(word.as_str());

        i = last_char;
        word = word.substring(0, (i - 1) as usize).to_string();
    }

    result
}

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
                print!("{}", compress(&buffer));
            } else if matches.is_present("decompress") {
                print!("{}", decompress(&buffer));
            }
        }
        Err(why) => panic!("{}", why),
    }
}
