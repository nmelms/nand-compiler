use crate::tokenizer::{TokenType, Tokenizer};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod compilation_engine;
mod tokenizer;

fn main() {
    let source = env::args().nth(1).expect("did not pass a source");
    let output = File::create("output.xml").expect("failed to create the output file");

    let mut tokenizer = Tokenizer::new(&source);
    let mut engine = compilation_engine::ComplationEngine::new(tokenizer, output);

    // let path = Path::new(&source);
}

// if path.is_file() {
//     let output_path = path.with_extension("xml");
//     let mut output = File::create(&output_path).expect("error creating output with path");
//     let mut tokenizer = Tokenizer::new(&source);
//     while tokenizer.has_more_tokens() {
//         println!("{}", tokenizer.current_token);
//         match tokenizer.current_token_type {
//             Some(TokenType::Identifier) => writeln!(
//                 output,
//                 "<identifier> {} </identifier>",
//                 tokenizer.current_token
//             )
//             .expect("failed to write"),
//             Some(TokenType::Keyword) => {
//                 writeln!(output, "<keyword> {} </keyword>", tokenizer.current_token)
//                     .expect("failed to write")
//             }
//             Some(TokenType::Symbol) => {
//                 writeln!(output, "<symbol> {} </symbol>", tokenizer.current_token)
//                     .expect("failed to write")
//             }
//             Some(TokenType::IntConst) => writeln!(
//                 output,
//                 "<integerConstant> {} </integerConstant>",
//                 tokenizer.current_token
//             )
//             .expect("failed to write"),
//             Some(TokenType::StringConst) => writeln!(
//                 output,
//                 "<stringConstant> {} </stringConstant>",
//                 tokenizer.current_token
//             )
//             .expect("failed to write"),
//             _ => println!("no match"),
//         }

//         tokenizer.advance();
//     }
// } else if path.is_dir() {
// for entry in fs::read_dir(path).expect("failed to read dir") {
//     println!("inside dir");
//     let entry = entry.expect("failed to read directory entry");
//     let file = entry.path();
//     let mut tokenizer = Tokenizer::new(file.to_str().expect("non utf8 path"));
//     let output_path = file.with_extension("xml");
//     let mut output = File::create(&output_path).expect("error creating output with path");

//     while tokenizer.has_more_tokens() {
//         println!("{}", tokenizer.current_token);

//         match tokenizer.current_token_type {
//             Some(TokenType::Identifier) => writeln!(
//                 output,
//                 "<identifier> {} </identifier>",
//                 tokenizer.current_token
//             )
//             .expect("failed to write"),
//             Some(TokenType::Keyword) => {
//                 writeln!(output, "<keyword> {} </keyword>", tokenizer.current_token)
//                     .expect("failed to write")
//             }
//             Some(TokenType::Symbol) => {
//                 writeln!(output, "<symbol> {} </symbol>", tokenizer.current_token)
//                     .expect("failed to write")
//             }
//             Some(TokenType::IntConst) => writeln!(
//                 output,
//                 "<integerConstant> {} </integerConstant>",
//                 tokenizer.current_token
//             )
//             .expect("failed to write"),
//             Some(TokenType::StringConst) => writeln!(
//                 output,
//                 "<stringConstant> {} </stringConstant>",
//                 tokenizer.current_token
//             )
//             .expect("failed to write"),
//             _ => println!("no match"),
//         }

//         tokenizer.advance();
//     }
// }
// }
