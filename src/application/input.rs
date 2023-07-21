use std::collections::HashMap;
use std::io::BufRead;

pub struct Input<R> {
    reader: R,
}

pub struct Summary<T> {
    pub text: String,
    pub frequencies: HashMap<T, u32>,
}

impl<R> Input<R>
where
    R: BufRead,
{
    pub fn from_source(source: R) -> Self {
        Input { reader: source }
    }

    pub fn process_as_chars(&mut self) -> Summary<char> {
        println!("Reading text from stdin into memory and computing character frequency.");
        let mut frequencies = HashMap::new();
        let mut line = String::new();
        let mut text = String::new();
        while let Ok(n_bytes) = self.reader.read_line(&mut line) {
            if n_bytes == 0 {
                break;
            }
            for c in line.chars().into_iter() {
                frequencies
                    .entry(c)
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
                text.push(c);
            }
            line.clear();
        }

        Summary { text, frequencies }
    }

    pub fn process_as_strings(&mut self) -> Summary<String> {
        println!("Reading text from stdin into memory and computing character frequency.");
        let mut frequencies = HashMap::new();
        let mut line = String::new();
        let mut text = String::new();
        while let Ok(n_bytes) = self.reader.read_line(&mut line) {
            if n_bytes == 0 {
                break;
            }
            for word in line.split_inclusive(" ").into_iter() {
                frequencies
                    .entry(String::from(word))
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
                text.push_str(word);
            }
            line.clear();
        }

        Summary {
            text: String::from(text),
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
        let mut input = Input::from_source(&input_text[..]);
        let char_summary = input.process_as_chars();
        assert_eq!(
            char_summary.text,
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
        let mut input = Input::from_source(&input_text[..]);
        let char_summary = input.process_as_strings();
        assert_eq!(
            char_summary.text,
            String::from_utf8(input_text.to_vec()).unwrap()
        );

        assert_eq!(char_summary.frequencies["Hello "], 2);
        assert_eq!(char_summary.frequencies["world! "], 1);
    }
}
