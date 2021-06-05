use std::char;
use substring::Substring;

pub fn compress(buf: &str) -> String {
    let mut line_prev = String::new();
    let mut result = String::new();

    for line in buf.lines() {
        let mut matches = 0;

        for chars in line.chars().zip(line_prev.chars()) {
            if chars.0 == chars.1 {
                matches += 1;
            } else {
                break;
            }
        }

        line_prev = line.to_string();
        result.push((matches + 1) as u8 as char);
        result.push_str(line.substring(matches, line.chars().count()));
    }

    result
}

pub fn decompress(buf: &str) -> String {
    let mut chars = buf.chars().peekable();
    let mut matches = chars.next().unwrap() as u8;
    let mut result = String::new();
    let mut word = String::new();

    while chars.peek().is_some() {
        matches -= 1;
        word = String::from(word.substring(0, matches as usize));

        for c in &mut chars {
            if c as u8 > 32 {
                word.push(c);
                matches += 1;
            } else {
                matches = c as u8;
                break;
            }
        }

        result.push_str(&word);
        result.push('\n');
    }

    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn compress_nothing() {
        assert_eq!(compress(""), "");
    }

    #[test]
    fn compress_unknown_chars() {
        assert_eq!(
            compress("😀\n😃\n😄\n😁\n😆\n😅\n😂\n🤣\n"), 
            "\u{1}😀\u{1}😃\u{1}😄\u{1}😁\u{1}😆\u{1}😅\u{1}😂\u{1}🤣"
        );
    }

    #[test]
    fn compress_unsorted_words() {
        assert_eq!(
            compress("beta\nalpha\ngamma\ndelta\nfoxtrot\ncharlie"),
            "\u{1}beta\u{1}alpha\u{1}gamma\u{1}delta\u{1}foxtrot\u{1}charlie"
        )
    }

    #[test]
    fn compress_a_few_words() {
        assert_eq!(
            compress("fabulous\nfailing\nfailure\nfault\nfederal\nfun\nfortnight\n"),
            "\u{1}fabulous\u{3}iling\u{5}ure\u{3}ult\u{2}ederal\u{2}un\u{2}ortnight"
        )
    }

    #[test]
    fn compress_decompress_english_dict() {
        if let Ok(input) = fs::read_to_string("tests/english.txt") {
            assert_eq!(decompress(&compress(&input)), input);
        }
    }

    #[test]
    fn compress_decompress_spanish_dict() {
        if let Ok(input) = fs::read_to_string("tests/spanish.txt") {
            assert_eq!(decompress(&compress(&input)), input);
        }
    }

    #[test]
    fn compress_decompress_french_dict() {
        if let Ok(input) = fs::read_to_string("tests/french.txt") {
            assert_eq!(decompress(&compress(&input)), input);
        }
    }
}
