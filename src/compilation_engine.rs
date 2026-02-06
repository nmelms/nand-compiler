use crate::tokenizer::{self, TokenType, Tokenizer};
use std::io::Write;
use std::{fs::File, process::Output};

pub struct ComplationEngine {
    tokenizer: Tokenizer,
    output: File,
}

impl ComplationEngine {
    pub fn new(tokenizer: Tokenizer, output: File) -> Self {
        let tokenizer = tokenizer;
        let output = output;

        Self { tokenizer, output }
    }

    pub fn comple_class(&mut self) {
        writeln!(self.output, "<class>").unwrap();
        self.process("class".to_string());
        self.process(self.tokenizer.current_token.clone());
        self.process("{".to_string());
        while self.tokenizer.current_token == "static" || self.tokenizer.current_token == "field" {
            self.compile_class_var_dec();
        }
        self.process("}".to_string());
        writeln!(self.output, "</class>").unwrap();
    }

    fn compile_class_var_dec(&mut self){
        writeln!(self.output, "<classVarDec>").unwrap();
        // static and field
        if self.tokenizer.current_token == "static"{
            self.process("static".to_string());
        }else if self.tokenizer.current_token == "field"{
            self.process("field".to_string());
        }
        // type
        match self.tokenizer.current_token.as_str() {
            "int" => self.process("int".to_string()),
            "char" => self.process("char".to_string()),
            "boolean" => self.process("boolean".to_string()),
            _ => {
                eprintln!("Syntax error: expected type (int | char | boolean), got '{}'", self.tokenizer.current_token);
                std::process::exit(1);
            }
        }
        self.process(self.tokenizer.current_token.clone());
        // identify multiple variables
        while self.tokenizer.current_token == ","{
            self.process(",".to_string());
            self.process(self.tokenizer.current_token.clone());

        }

        self.process(";".to_string());

        writeln!(self.output, "</classVarDec>").unwrap();
    }

    fn process(&mut self, token: String) {
        println!("{} {}", &self.tokenizer.current_token, &token);
        if &self.tokenizer.current_token == &token {
            let cur_token = self.tokenizer.current_token.clone();
            let cur_type = self.tokenizer.current_token_type;
            self.print_xml_token(&cur_token, cur_type);
        } else {
            println!("syntax error")
        }
        &self.tokenizer.advance();
    }

    fn print_xml_token(&mut self, current_token: &String, current_token_type: Option<TokenType>) {
        match current_token_type {
            Some(TokenType::Identifier) => {
                writeln!(self.output, "<identifier> {} </identifier>", current_token)
                    .expect("failed to write")
            }
            Some(TokenType::Keyword) => {
                writeln!(self.output, "<keyword> {} </keyword>", current_token)
                    .expect("failed to write")
            }
            Some(TokenType::Symbol) => {
                writeln!(self.output, "<symbol> {} </symbol>", current_token)
                    .expect("failed to write")
            }
            Some(TokenType::IntConst) => writeln!(
                self.output,
                "<integerConstant> {} </integerConstant>",
                current_token
            )
            .expect("failed to write"),
            Some(TokenType::StringConst) => writeln!(
                self.output,
                "<stringConstant> {} </stringConstant>",
                current_token
            )
            .expect("failed to write"),
            _ => println!("no match"),
        }
    }
}
