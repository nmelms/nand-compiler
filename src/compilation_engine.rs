use std::{fs::File, process::Output};
use crate::tokenizer::{self, TokenType, Tokenizer};
use std::io::Write;



pub struct ComplationEngine{
    tokenizer: Tokenizer,
    output: File,
}


impl ComplationEngine{

    pub fn new(tokenizer: Tokenizer, output: File) -> Self{
        let tokenizer = tokenizer;
        let output = output;

        Self { tokenizer, output }   
    }

    pub fn comple_class(&self){
        writeln!(output, "<Class>")
        

    }
    // todo
    // finish implemting process and compelete complie_class functioncpocod
    fn process(&mut self, token: String ) {
        if &self.tokenizer.current_token == &token{
            let cur_token = self.tokenizer.current_token.clone();
            let cur_type  = self.tokenizer.current_token_type; 
            self.print_xml_token(&cur_token, cur_type);
        }else{
            println!("syntax error") 
        }
        &self.tokenizer.advance();

    }

    fn print_xml_token(&mut self, current_token: &String, current_token_type: Option<TokenType>){

            match current_token_type {
                Some(TokenType::Identifier) => writeln!(
                    self.output,
                    "<identifier> {} </identifier>",
                    current_token
                )
                .expect("failed to write"),
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

        todo!("implemnt printXMLToken")
    }   


}