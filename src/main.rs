use crate::tokenizer::{TokenType, Tokenizer};
use std::fs;
use std::io::Write;

mod tokenizer;

fn main() {
    println!("running in main");
    let mut tokenizer = Tokenizer::new("test.jack");
    let mut output = fs::File::create("outputT.xml").expect("failed to create file");

    while tokenizer.has_more_tokens() {
        println!("{}", tokenizer.current_token);

        match tokenizer.current_token_type {
            Some(TokenType::Identifier) => writeln!(
                output,
                "<identifier>{}</identifier>",
                tokenizer.current_token
            )
            .expect("failed to write"),
            Some(TokenType::Keyword) => {
                writeln!(output, "<keyword>{}</keyword>", tokenizer.current_token)
                    .expect("failed to write")
            }
            Some(TokenType::Symbol) => {
                writeln!(output, "<symbol>{}</symbol>", tokenizer.current_token)
                    .expect("failed to write")
            }
            Some(TokenType::IntConst) => writeln!(
                output,
                "<intergerConstant>{}</intergetConstant>",
                tokenizer.current_token
            )
            .expect("failed to write"),
            Some(TokenType::StringConst) => writeln!(
                output,
                "<stringConstant>{}</stringConstant>",
                tokenizer.current_token
            )
            .expect("failed to write"),
            _ => println!("no match"),
        }

        tokenizer.advance();
    }
}
