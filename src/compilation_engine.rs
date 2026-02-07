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
        if self.tokenizer.current_token_type == Some(TokenType::Identifier) {
            self.process(self.tokenizer.current_token.clone());
        } else {
            eprintln!(
                "Syntax error: expected className identifier, got '{}'",
                self.tokenizer.current_token
            );
            std::process::exit(1);
        }

        self.process("{".to_string());
        while self.tokenizer.current_token == "static" || self.tokenizer.current_token == "field" {
            self.compile_class_var_dec();
        }
        while self.tokenizer.current_token == "constructor"
            || self.tokenizer.current_token == "function"
            || self.tokenizer.current_token == "method"
        {
            self.compile_subroutine();
        }

        self.process("}".to_string());

        writeln!(self.output, "</class>").unwrap();
    }

    fn compile_subroutine(&mut self) {
        //
        match self.tokenizer.current_token.as_str() {
            "constructor" => self.process("constructor".to_string()),
            "function" => self.process("function".to_string()),
            "method" => self.process("method".to_string()),
            _ => {
                eprintln!(
                    "Syntax error: expected (contructor | function | method), got '{}'",
                    self.tokenizer.current_token
                );
                std::process::exit(1);
            }
        }
        // params
        // ('void|type)
        match self.tokenizer.current_token_type {
            Some(TokenType::Keyword) => match self.tokenizer.current_token.as_str() {
                "void" => self.process(self.tokenizer.current_token.to_string()),
                "int" => self.process("int".to_string()),
                "char" => self.process("char".to_string()),
                "boolean" => self.process("boolean".to_string()),
                _ => print!("something fucked up"),
            },
            Some(TokenType::Identifier) => self.process(self.tokenizer.current_token.to_string()),
            None => {
                eprintln!("Syntax error: got '{}'", self.tokenizer.current_token);
                std::process::exit(1);
            }
            _ => {
                eprintln!(
                    "Syntax error: expected (void | type), got '{}'",
                    self.tokenizer.current_token
                );
                std::process::exit(1);
            }
        }
        // subroutine name
        if self.tokenizer.current_token_type == Some(TokenType::Identifier) {
            self.process(self.tokenizer.current_token.to_string());
        } else {
            println!(
                "Syntax error: expected subroutine name '{}'",
                self.tokenizer.current_token
            );
            std::process::exit(1);
        }
        self.process("(".to_string());
        self.compile_parameter_list();
        self.process(")".to_string());
        self.compile_subroutine_body();
        self.compile_statements();
        self.process("}".to_string());
    }

    fn compile_subroutine_body(&mut self) {
        self.process("{".to_string());
        while self.tokenizer.current_token == "var" {
            self.compile_var_dec();
        }
    }

    fn compile_statements(&mut self) {
        while self.tokenizer.current_token == "let".to_string()
            || self.tokenizer.current_token == "if".to_string()
            || self.tokenizer.current_token == "while".to_string()
            || self.tokenizer.current_token == "do".to_string()
            || self.tokenizer.current_token == "return".to_string()
        {
            match self.tokenizer.current_token.as_str() {
                "let" => self.compile_let(),
                // "if" => self.compile_if(),
                // "while" => self.compile_while(),
                // "do" => self.compile_do(),
                _ => println!("the end"),
            }
        }
    }

    fn compile_let(&mut self) {
        self.process("let".to_string());
        // varName
        if self.tokenizer.current_token_type == Some(TokenType::Identifier) {
            self.process(self.tokenizer.current_token.to_string());
        }
        // [ expression ]
        if self.tokenizer.current_token == "[" {
            self.process("[".to_string());
            self.compile_expression();
            self.process("]".to_string());
        }

        self.process("=".to_string());
        self.compile_expression();
        self.process(";".to_string())
    }

    fn compile_expression(&mut self) {
        self.compile_term();

        while matches!(
            self.tokenizer.current_token.as_str(),
            "+" | "-" | "*" | "/" | "|" | "=" | "&lt;" | "&gt;" | "&amp;"
        ) {
            self.process(self.tokenizer.current_token.to_string());
            self.compile_term();
        }
    }

    fn compile_term(&mut self) {
        match self.tokenizer.current_token_type {
            Some(TokenType::IntConst) => {
                self.process(self.tokenizer.current_token.to_string());
            }
            Some(TokenType::StringConst) => {
                self.process(self.tokenizer.current_token.to_string());
            }
            Some(TokenType::Keyword) => match self.tokenizer.current_token.as_str() {
                "true" => self.process(self.tokenizer.current_token.to_string()),
                "false" => self.process(self.tokenizer.current_token.to_string()),
                "null" => self.process(self.tokenizer.current_token.to_string()),
                "this" => self.process(self.tokenizer.current_token.to_string()),
                _ => {
                    eprintln!(
                        "Syntax error: expected (true | false | null | this), got '{}'",
                        self.tokenizer.current_token
                    );
                    std::process::exit(1);
                }
            },
            Some(TokenType::Identifier) => {
                self.process(self.tokenizer.current_token.to_string());
                match self.tokenizer.current_token.as_str() {
                    "[" => {
                        self.process("[".to_string());
                        self.compile_expression();
                        self.process("]".to_string())
                    }
                    // subroutine call
                    "(" => {
                        self.process("(".to_string());
                        self.compile_expression_list();
                        self.process(")".to_string())
                    }
                    // other type of subroutiner call
                    "." => {
                        self.process(".".to_string());
                        if self.tokenizer.current_token_type == Some(TokenType::Identifier) {
                            self.process(self.tokenizer.current_token.to_string());
                        } else {
                            eprintln!(
                                "Syntax error: expected subroutineName, got '{}'",
                                self.tokenizer.current_token
                            );
                            std::process::exit(1);
                        }
                        self.process("(".to_string());
                        self.compile_expression_list();
                        self.process(")".to_string())
                    }
                    _ => return,
                }
            }
            Some(TokenType::Symbol) => match self.tokenizer.current_token.as_str() {
                // unary op
                "-" => self.process(self.tokenizer.current_token.to_string()),
                "~" => self.process(self.tokenizer.current_token.to_string()),
                "(" => {
                    self.process("(".to_string());
                    self.compile_expression();
                    self.process(")".to_string())
                }

                _ => {
                    eprintln!(
                        "Syntax error: expected a unaryOp, got '{}'",
                        self.tokenizer.current_token
                    );
                    std::process::exit(1);
                }
            },

            _ => print!("error"),
        }
    }

    fn compile_expression_list(&mut self) -> i32 {
        let mut total = 0;
        if self.tokenizer.current_token != ")".to_string() {
            total += 1;
            self.compile_expression();
            while self.tokenizer.current_token == ','.to_string() {
                self.process(",".to_string());
                total += 1;
                self.compile_expression();
            }
            return total;
        } else {
            return total;
        }
    }

    fn compile_var_dec(&mut self) {
        self.process("var".to_string());
        // (type)
        match self.tokenizer.current_token_type {
            Some(TokenType::Keyword) => match self.tokenizer.current_token.as_str() {
                "int" => self.process("int".to_string()),
                "char" => self.process("char".to_string()),
                "boolean" => self.process("boolean".to_string()),
                _ => print!("something fucked up"),
            },
            // this if for classname
            Some(TokenType::Identifier) => self.process(self.tokenizer.current_token.to_string()),
            _ => {
                eprintln!("Syntax error: got '{}'", self.tokenizer.current_token);
                std::process::exit(1);
            }
        }

        // varname

        // first varName
        if self.tokenizer.current_token_type == Some(TokenType::Identifier) {
            self.process(self.tokenizer.current_token.to_string());
        } else {
            eprintln!(
                "Syntax error: expected varName, got '{}'",
                self.tokenizer.current_token
            );
            std::process::exit(1);
        }

        while self.tokenizer.current_token == "," {
            self.process(",".to_string());
            match self.tokenizer.current_token_type {
                Some(TokenType::Identifier) => {
                    self.process(self.tokenizer.current_token.to_string())
                }
                _ => {
                    eprintln!("Syntax error: got '{}'", self.tokenizer.current_token);
                    std::process::exit(1);
                }
            }
        }

        self.process(";".to_string());
    }

    fn compile_parameter_list(&mut self) {
        if self.tokenizer.current_token != ")" {
            // Process type
            match self.tokenizer.current_token_type {
                Some(TokenType::Identifier) => {
                    self.process(self.tokenizer.current_token.to_string())
                }
                _ => match self.tokenizer.current_token.as_str() {
                    "int" => self.process("int".to_string()),
                    "char" => self.process("char".to_string()),
                    "boolean" => self.process("boolean".to_string()),
                    _ => {
                        println!(
                            "Syntax error: expected subroutine name '{}'",
                            self.tokenizer.current_token
                        );
                        std::process::exit(1);
                    }
                },
            }
            //process varname
            self.process(self.tokenizer.current_token.to_string());
            while self.tokenizer.current_token == "," {
                self.process(",".to_string());
                // Process type
                match self.tokenizer.current_token_type {
                    Some(TokenType::Identifier) => {
                        self.process(self.tokenizer.current_token.to_string())
                    }
                    _ => match self.tokenizer.current_token.as_str() {
                        "int" => self.process("int".to_string()),
                        "char" => self.process("char".to_string()),
                        "boolean" => self.process("boolean".to_string()),
                        _ => {
                            println!(
                                "Syntax error: expected subroutine name '{}'",
                                self.tokenizer.current_token
                            );
                            std::process::exit(1);
                        }
                    },
                }
                //process varname
                self.process(self.tokenizer.current_token.to_string());
            }
        }
    }

    fn compile_class_var_dec(&mut self) {
        writeln!(self.output, "<classVarDec>").unwrap();
        // static and field
        if self.tokenizer.current_token == "static" {
            self.process("static".to_string());
        } else if self.tokenizer.current_token == "field" {
            self.process("field".to_string());
        }
        // type
        match self.tokenizer.current_token.as_str() {
            "int" => self.process("int".to_string()),
            "char" => self.process("char".to_string()),
            "boolean" => self.process("boolean".to_string()),
            _ => {
                eprintln!(
                    "Syntax error: expected type (int | char | boolean), got '{}'",
                    self.tokenizer.current_token
                );
                std::process::exit(1);
            }
        }
        self.process(self.tokenizer.current_token.clone());
        // identify multiple variables
        while self.tokenizer.current_token == "," {
            self.process(",".to_string());
            self.process(self.tokenizer.current_token.clone());
        }

        self.process(";".to_string());

        writeln!(self.output, "</classVarDec>").unwrap();
    }

    fn process(&mut self, token: String) {
        if self.tokenizer.current_token == token {
            let cur_token = self.tokenizer.current_token.clone();
            let cur_type = self.tokenizer.current_token_type;
            self.print_xml_token(&cur_token, cur_type);
            self.tokenizer.advance();
        } else {
            eprintln!(
                "Syntax error: expected '{}', got '{}'",
                token, self.tokenizer.current_token
            );
            std::process::exit(1);
        }
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
