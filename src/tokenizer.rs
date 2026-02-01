use std::fs;

pub struct Tokenizer {
    chars: Vec<char>,
    i: usize,
    current_token: Option<Token>,
}

pub enum Token {

}

impl Tokenizer {
    pub fn new(path: &str) -> Self {
        let chars = std::fs::read_to_string(path)
            .expect("Failed to read .jack file")
            .chars()
            .collect();
        let i = 0;
        let current_token = None;

        println!("{:?}", chars);
        Self { chars, i, current_token }

        
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

        while self.chars[self.i] != ' ' || self.chars[self.i] != ';' {
            s.push(self.chars[self.i]);
            self.i += 1;
        }
    }
}
