use std::char;
use substring::Substring;

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

        result.push((matches + 1) as u8 as char);
        result.push_str(line.substring(matches, len));

        line_prev = line.to_string();
    }

    result
}

pub fn decompress(buf: &str) -> String {
    let mut chars = buf.chars().peekable();
    let mut last_char = 0u8;
    let mut matches = chars.next().unwrap() as u8;
    let mut result = String::new();
    let mut word = String::new();

    while chars.peek().is_some() {
        matches -= 1;
        word = String::from(word.substring(0, matches as usize));

        for c in &mut chars {
            last_char = c as u8;
            if last_char > 32 {
                word.push(c);
                matches += 1;
            } else {
                break;
            }
        }

        matches = last_char;

        result.push_str(&word);
        result.push('\n');
    }

    result
}
