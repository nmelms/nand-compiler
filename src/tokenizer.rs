use std::fs;

pub struct Tokenizer {
    chars: Vec<char>,
    i: usize,
    pub current_token: String,
    pub current_token_type: Option<TokenType>,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Keyword,
    Symbol,
    Identifier,
    IntConst,
    StringConst,
}

const KEYWORDS: [&str; 21] = [
    "class",
    "constructor",
    "function",
    "method",
    "field",
    "static",
    "var",
    "int",
    "char",
    "boolean",
    "void",
    "true",
    "false",
    "null",
    "this",
    "let",
    "do",
    "if",
    "else",
    "while",
    "return",
];
impl Tokenizer {
    pub fn new(path: &str) -> Self {
        let chars = std::fs::read_to_string(path)
            .expect("Failed to read .jack file")
            .chars()
            .collect();
        let i = 0;
        let current_token = String::from("None");
        let current_token_type = None;

        Self {
            chars,
            i,
            current_token,
            current_token_type,
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

        // consume keyword or identifier
        if self.chars[self.i].is_ascii_alphabetic() {
            while self.chars[self.i].is_ascii_alphabetic() {
                s.push(self.chars[self.i]);
                self.i += 1;
            }
            if KEYWORDS.contains(&s.as_str()) {
                self.current_token_type = Some(TokenType::Keyword);
            } else {
                self.current_token_type = Some(TokenType::Identifier);
            }
            self.current_token = s;
            return;
        }

        //consume symbol
        if Tokenizer::is_symbol(self.chars[self.i]) {
            let special_symbols = &['<', '>', '"', '&'];

            if (special_symbols.contains(&self.chars[self.i])) {
                match &self.chars[self.i] {
                    '<' => self.current_token = "&lt;".to_string(),
                    '>' => self.current_token = "&gt;".to_string(),
                    '"' => self.current_token = "&quot;".to_string(),
                    '&' => self.current_token = "amp;".to_string(),
                    _ => println!("something weird. happened"),
                }
            } else {
                self.current_token = self.chars[self.i].to_string();
            }
            self.current_token_type = Some(TokenType::Symbol);

            self.i += 1;
            return;
        }

        //consume int
        if self.chars[self.i].is_ascii_digit() {
            while self.chars[self.i].is_ascii_digit() {
                s.push(self.chars[self.i]);
                self.i += 1;
            }
            self.current_token_type = Some(TokenType::IntConst);
            self.current_token = s;
            return;
        }

        // consume string
        if self.chars[self.i] == '"' {
            let special_symbols = &['<', '>', '"', '&'];
            self.i += 1;
            while self.chars[self.i] != '"' {
                if special_symbols.contains(&self.chars[self.i]) {
                    match &self.chars[self.i] {
                        '<' => s.push_str("&lt;"),
                        '>' => s.push_str("&gt;"),
                        '"' => s.push_str("&quot;"),
                        '&' => s.push_str("&amp;"),
                        _ => println!("something weird. happened"),
                    }
                } else {
                    s.push(self.chars[self.i]);
                }
                
                self.i += 1;
            }
            self.i += 1;
            self.current_token_type = Some(TokenType::StringConst);
            self.current_token = s;
            return;
        }
    }
    pub fn keyword(&self) -> Option<&TokenType> {
        match self.current_token_type.as_ref() {
            Some(TokenType::Keyword) => self.current_token_type.as_ref(),
            _ => None,
        }
    }

    pub fn symbol(&self) -> Option<&TokenType> {
        match self.current_token_type.as_ref() {
            Some(TokenType::Symbol) => self.current_token_type.as_ref(),
            _ => None,
        }
    }

    pub fn identifier(&self) -> Option<&TokenType> {
        match self.current_token_type.as_ref() {
            Some(TokenType::Identifier) => self.current_token_type.as_ref(),
            _ => None,
        }
    }
    pub fn intVal(&self) -> Option<&TokenType> {
        match self.current_token_type.as_ref() {
            Some(TokenType::IntConst) => self.current_token_type.as_ref(),
            _ => None,
        }
    }

    pub fn stringVal(&self) -> Option<&TokenType> {
        match self.current_token_type.as_ref() {
            Some(TokenType::StringConst) => self.current_token_type.as_ref(),
            _ => None,
        }
    }

    fn is_symbol(cur: char) -> bool {
        "{}()[].,;+-*/&|<>=~".contains(cur)
    }
}
