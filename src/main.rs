use crate::tokenizer::Tokenizer;

mod tokenizer;

fn main() {
    println!("running in main");

    let mut tokenizer = Tokenizer::new("test.jack");
    tokenizer.advance();
    tokenizer.advance();
    tokenizer.advance();


}
