use crate::tokenizer::Tokenizer;

mod tokenizer;

fn main() {
    println!("running in main");

    let mut tokenizer = Tokenizer::new("test.jack");
    println!("{}", tokenizer.current_token);
    tokenizer.advance();
    println!("{}", tokenizer.current_token);

    tokenizer.advance();
    println!("{}", tokenizer.current_token);
    tokenizer.advance();
    println!("{}", tokenizer.current_token);

        tokenizer.advance();
    println!("{}", tokenizer.current_token);

            tokenizer.advance();
    println!("{}", tokenizer.current_token);

            tokenizer.advance();
    println!("{}", tokenizer.current_token);
}
