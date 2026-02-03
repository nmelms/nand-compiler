use crate::tokenizer::Tokenizer;

mod tokenizer;

fn main() {
    println!("running in main");
        let mut tokenizer = Tokenizer::new("test.jack");

    while tokenizer.has_more_tokens(){
        println!("{}", tokenizer.current_token);
        tokenizer.advance();
    }

}
