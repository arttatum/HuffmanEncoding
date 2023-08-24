use std::collections::HashMap;
use std::io::BufRead;

/// Parses input from implementor of BufRead.
/// Computes frequency of tokens (chars / words) and stores entire input in memory.
pub struct TokenParser<T> {
    pub lines: Vec<String>,
    pub token_frequencies: HashMap<T, u32>,
}

impl TokenParser<char> {
    /// Parse input into `Vec<String>` lines, and compute frequency of each char in the input.
    pub fn chars_from_reader<R: BufRead>(mut reader: R) -> Self {
        let mut token_frequencies = HashMap::new();
        let mut line = String::new();
        let mut lines = Vec::new();
        while let Ok(n_bytes) = reader.read_line(&mut line) {
            if n_bytes == 0 {
                break;
            }
            for c in line.chars().into_iter() {
                token_frequencies
                    .entry(c)
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
            }
            lines.push(line.clone());
            line.clear();
        }

        TokenParser {
            lines,
            token_frequencies,
        }
    }
}

impl TokenParser<String> {
    /// Parse input into `Vec<String>` lines, and compute frequency of each word in the input.
    pub fn words_from_reader<R: BufRead>(mut reader: R) -> Self {
        let mut token_frequencies = HashMap::new();
        let mut line = String::new();
        let mut lines = Vec::new();
        while let Ok(n_bytes) = reader.read_line(&mut line) {
            if n_bytes == 0 {
                break;
            }
            for word in line.split_inclusive(" ").into_iter() {
                token_frequencies
                    .entry(String::from(word))
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
            }
            lines.push(line.clone());
            line.clear();
        }

        TokenParser {
            lines,
            token_frequencies,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_as_chars() {
        let lines_text = b"Hello world!\nGoodbye :(";
        let char_summary = TokenParser::chars_from_reader(&lines_text[..]);
        assert_eq!(char_summary.lines, vec!["Hello world!\n", "Goodbye :("]);
        assert_eq!(char_summary.token_frequencies[&'H'], 1);
        assert_eq!(char_summary.token_frequencies[&'e'], 2);
        assert_eq!(char_summary.token_frequencies[&'l'], 3);
        assert_eq!(char_summary.token_frequencies[&'o'], 4);
        assert_eq!(char_summary.token_frequencies[&'w'], 1);
        assert_eq!(char_summary.token_frequencies[&'r'], 1);
        assert_eq!(char_summary.token_frequencies[&'d'], 2);
        assert_eq!(char_summary.token_frequencies[&'!'], 1);
        assert_eq!(char_summary.token_frequencies[&'\n'], 1);
        assert_eq!(char_summary.token_frequencies[&'G'], 1);
        assert_eq!(char_summary.token_frequencies[&'b'], 1);
        assert_eq!(char_summary.token_frequencies[&'y'], 1);
        assert_eq!(char_summary.token_frequencies[&' '], 2);
        assert_eq!(char_summary.token_frequencies[&':'], 1);
        assert_eq!(char_summary.token_frequencies[&'('], 1);
    }

    #[test]
    fn test_process_as_strings() {
        let lines_text = b"Hello world! \n Hello ";
        let str_summary = TokenParser::words_from_reader(&lines_text[..]);
        assert_eq!(str_summary.lines, vec!["Hello world! \n", " Hello "]);
        assert_eq!(str_summary.token_frequencies["Hello "], 2);
        assert_eq!(str_summary.token_frequencies["world! "], 1);
    }
}
