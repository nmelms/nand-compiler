use std::fs;

pub struct Tokenizer {
    chars: Vec<char>,
    i: usize,
    pub current_token: String,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum TokenType {
//     Keyword,
//     Symbol,
//     Identifier,
//     IntConst,
//     StringConst,
//     None,
// }

impl Tokenizer {
    pub fn new(path: &str) -> Self {
        let chars = std::fs::read_to_string(path)
            .expect("Failed to read .jack file")
            .chars()
            .collect();
        let i = 0;
        let current_token = String::from("None");

        Self {
            chars,
            i,
            current_token,
        }
    }

    pub fn has_more_tokens(&self) -> bool {
        if self.i < self.chars.len() {
            true
        } else {
            return false;
        }
    }

    pub fn advance(&mut self) {
        let mut s: String = String::from("");

        // skip spaces
        while self.chars[self.i] == ' ' || self.chars[self.i] == '\n' {
            self.i += 1;
        }

        // consume keyword
        if self.chars[self.i].is_ascii_alphabetic() {
            while self.chars[self.i].is_ascii_alphabetic() {
                s.push(self.chars[self.i]);
                self.i += 1;
            }
            self.current_token = s;
        }

        //consume symbol
        if Tokenizer::is_symbol(self.chars[self.i]) {
            self.current_token = self.chars[self.i].to_string();
            self.i += 1;
        }
    }

    fn is_symbol(cur: char) -> bool {
        "{}()[].,;+-*/&|<>=~".contains(cur)
    }
}
