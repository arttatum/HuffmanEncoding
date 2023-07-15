use std::collections::HashMap;
use std::io::BufRead;
pub struct TextProvider<TokenType> {
    text: String,
    frequencies: HashMap<TokenType, u32>,
}

impl TextProvider<char> {
    pub fn from_stdin() -> Self {
        println!("Reading text from stdin into memory and computing character frequency.");
        let mut frequencies = HashMap::new();
        let mut line = String::new();
        let mut text = String::new();
        while let Ok(n_bytes) = std::io::stdin().lock().read_line(&mut line) {
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

        TextProvider { text, frequencies }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_frequencies(&self) -> &HashMap<char, u32> {
        &self.frequencies
    }
}
