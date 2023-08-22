use std::collections::HashMap;
use std::io::BufRead;

pub struct Summary<T> {
    pub input: String,
    pub frequencies: HashMap<T, u32>,
}

impl Summary<char> {
    pub fn chars_from_reader<R: BufRead>(mut reader: R) -> Self {
        let mut frequencies = HashMap::new();
        let mut line = String::new();
        let mut input = String::new();
        while let Ok(n_bytes) = reader.read_line(&mut line) {
            if n_bytes == 0 {
                break;
            }
            for c in line.chars().into_iter() {
                frequencies
                    .entry(c)
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
                input.push(c);
            }
            line.clear();
        }

        Summary { input, frequencies }
    }
}

impl Summary<String> {
    pub fn strs_from_reader<R: BufRead>(mut reader: R) -> Self {
        let mut frequencies = HashMap::new();
        let mut line = String::new();
        let mut input = String::new();
        while let Ok(n_bytes) = reader.read_line(&mut line) {
            if n_bytes == 0 {
                break;
            }
            for word in line.split_inclusive(" ").into_iter() {
                frequencies
                    .entry(String::from(word))
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
                input.push_str(word);
            }
            line.clear();
        }

        Summary {
            input: String::from(input),
            frequencies,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_as_chars() {
        let input_text = b"Hello world!";
        let char_summary = Summary::chars_from_reader(&input_text[..]);
        assert_eq!(
            char_summary.input,
            String::from_utf8(input_text.to_vec()).unwrap()
        );
        assert_eq!(char_summary.frequencies[&'H'], 1);
        assert_eq!(char_summary.frequencies[&'e'], 1);
        assert_eq!(char_summary.frequencies[&'l'], 3);
        assert_eq!(char_summary.frequencies[&'o'], 2);
        assert_eq!(char_summary.frequencies[&'w'], 1);
        assert_eq!(char_summary.frequencies[&'r'], 1);
        assert_eq!(char_summary.frequencies[&'d'], 1);
        assert_eq!(char_summary.frequencies[&'!'], 1);
    }

    #[test]
    fn test_process_as_strings() {
        let input_text = b"Hello world! Hello ";
        let str_summary = Summary::strs_from_reader(&input_text[..]);
        assert_eq!(
            str_summary.input,
            String::from_utf8(input_text.to_vec()).unwrap()
        );

        assert_eq!(str_summary.frequencies["Hello "], 2);
        assert_eq!(str_summary.frequencies["world! "], 1);
    }
}
