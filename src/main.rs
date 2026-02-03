use crate::tokenizer::{TokenType, Tokenizer};

mod tokenizer;

fn main() {
    println!("running in main");
    let mut tokenizer = Tokenizer::new("test.jack");

    while tokenizer.has_more_tokens() {
        println!("{}", tokenizer.current_token);
        if let Some(TokenType::Keyword) = tokenizer.current_token_type.as_ref() {
            println!("it was a keyword: {}", tokenizer.current_token);
        }

        tokenizer.advance();
    }
}
