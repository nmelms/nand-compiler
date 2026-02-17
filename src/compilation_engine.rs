use crate::symbol_table::{self, SymbolTable};
use crate::tokenizer::{self, TokenType, Tokenizer};
use crate::vm_writer::{self, VMWriter};
use std::io::Write;
use std::{fs::File, process::Output};

pub struct ComplationEngine {
    tokenizer: Tokenizer,
    symbol_table: SymbolTable,
    sub_symbol_table: SymbolTable,
    pub class_name: String,
    indent: usize,
    vm_writer: VMWriter,
}

impl ComplationEngine {
    pub fn new(tokenizer: Tokenizer, output: File) -> Self {
        let tokenizer = tokenizer;
        let symbol_table = SymbolTable::new();
        let sub_symbol_table = SymbolTable::new();
        let class_name = String::new();
        let vm_writer = VMWriter::new(output);

        Self {
            tokenizer,
            indent: 0,
            symbol_table,
            sub_symbol_table,
            class_name,
            vm_writer,
        }
    }

    pub fn comple_class(&mut self) {
        self.process("class".to_string());
        if self.tokenizer.current_token_type == Some(TokenType::Identifier) {
            // save Main classname to use later
            self.class_name = self.tokenizer.current_token.clone();
            self.process(self.tokenizer.current_token.clone());
        } else {
            eprintln!(
                "Syntax error: expected className identifier, got '{}'",
                self.tokenizer.current_token
            );
            std::process::exit(1);
        }

        self.process("{".to_string());
        println!(
            "after process curely currenmt toke: {}",
            self.tokenizer.current_token
        );
        while self.tokenizer.current_token == "static" || self.tokenizer.current_token == "field" {
            self.compile_class_var_dec();
        }
        while self.tokenizer.current_token == "constructor"
            || self.tokenizer.current_token == "function"
            || self.tokenizer.current_token == "method"
        {
            println!("inside con fun meth");
            self.compile_subroutine();
        }

        self.process("}".to_string());
    }

    fn compile_subroutine(&mut self) {
        println!("compeile subroutine is running");
        self.sub_symbol_table.reset();

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
        let function_name;
        if self.tokenizer.current_token_type == Some(TokenType::Identifier) {
            function_name = self.tokenizer.current_token.clone();
            self.process(self.tokenizer.current_token.to_string());
        } else {
            println!(
                "Syntax error: expected subroutine name '{}'",
                self.tokenizer.current_token
            );
            std::process::exit(1);
        }

        let full_name = format!("{}.{}", self.class_name, function_name);

        self.process("(".to_string());
        self.compile_parameter_list();
        self.process(")".to_string());

        // body
        self.process("{".to_string());
        while self.tokenizer.current_token == "var" {
            self.compile_var_dec(); // make varDec wrapped too
        }
        self.vm_writer.write_function(
            &full_name,
            self.sub_symbol_table
                .var_count(symbol_table::SymbolType::Var),
        );
        self.compile_statements();

        self.process("}".to_string());
    }

    // fn compile_subroutine_body(&mut self) {
    //     self.open_tag("subroutineBody");
    //     self.process("{".to_string());
    //     while self.tokenizer.current_token == "var" {
    //         self.compile_var_dec();
    //     }
    //     self.compile_statements();
    //     self.process("}".to_string());
    //     self.close_tag("subroutineBody");
    // }

    fn compile_statements(&mut self) {
        println!("inside compile statemetns");
        self.open_tag("statements");
        while self.tokenizer.current_token == "let".to_string()
            || self.tokenizer.current_token == "if".to_string()
            || self.tokenizer.current_token == "while".to_string()
            || self.tokenizer.current_token == "do".to_string()
            || self.tokenizer.current_token == "return".to_string()
        {
            match self.tokenizer.current_token.as_str() {
                "let" => self.compile_let(),
                "if" => self.compile_if(),
                "while" => self.compile_while(),
                "do" => self.compile_do(),
                "return" => self.compile_return(),
                _ => println!("the end"),
            }
        }
        self.close_tag("statements");
    }

    fn compile_let(&mut self) {
        self.open_tag("letStatement");
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
        self.process(";".to_string());
        self.close_tag("letStatement");
    }

    fn compile_if(&mut self) {
        self.open_tag("ifStatement");
        self.process("if".to_string());
        self.process("(".to_string());
        self.compile_expression();
        self.process(")".to_string());
        self.process("{".to_string());
        self.compile_statements();
        self.process("}".to_string());
        if self.tokenizer.current_token == "else".to_string() {
            self.process("else".to_string());
            self.process("{".to_string());
            self.compile_statements();
            self.process("}".to_string());
        }
        self.close_tag("ifStatement");
    }

    fn compile_return(&mut self) {
        self.process("return".to_string());
        if self.starts_expression() {
            self.compile_expression();
        } else {
            self.vm_writer.write_push("constant", 0);
        }
        self.vm_writer.write_return();
        self.process(";".to_string());
    }

    fn starts_expression(&self) -> bool {
        match self.tokenizer.current_token_type {
            Some(TokenType::IntConst) => true,
            Some(TokenType::StringConst) => true,
            Some(TokenType::Identifier) => true,
            Some(TokenType::Keyword) => matches!(
                self.tokenizer.current_token.as_str(),
                "true" | "false" | "null" | "this"
            ),
            Some(TokenType::Symbol) => {
                matches!(self.tokenizer.current_token.as_str(), "(" | "-" | "~")
            }
            _ => false,
        }
    }

    fn compile_do(&mut self) {
        // todo!("create compile do in VM code")
        let mut function_name = String::new();
        let mut num_args: i32 = 0;
        self.process("do".to_string());
        // subroutine
        match self.tokenizer.current_token_type {
            Some(TokenType::Identifier) => {
                function_name = function_name + &self.tokenizer.current_token;

                self.process(self.tokenizer.current_token.to_string());
                match self.tokenizer.current_token.as_str() {
                    // subroutine call
                    "(" => {
                        self.process("(".to_string());
                        num_args = self.compile_expression_list();
                        self.process(")".to_string())
                    }
                    // other type of subroutine call
                    "." => {
                        function_name = function_name + ".";
                        self.process(".".to_string());
                        if self.tokenizer.current_token_type == Some(TokenType::Identifier) {
                            function_name = function_name + &self.tokenizer.current_token;
                            self.process(self.tokenizer.current_token.to_string());
                        } else {
                            eprintln!(
                                "Syntax error: expected subroutineName, got '{}'",
                                self.tokenizer.current_token
                            );
                            std::process::exit(1);
                        }
                        self.process("(".to_string());
                        num_args = self.compile_expression_list();
                        self.process(")".to_string())
                    }
                    _ => println!("check in compile do. Unknown error"),
                }
            }
            _ => println!("check in compile do. Unknown error"),
        }
        self.vm_writer.write_call(&function_name, num_args);
        self.vm_writer.write_pop("temp", 0);
        self.process(";".to_string());
        self.close_tag("doStatement");
    }

    fn compile_while(&mut self) {
        self.open_tag("whileStatement");
        self.process("while".to_string());
        self.process("(".to_string());
        self.compile_expression();
        self.process(")".to_string());
        self.process("{".to_string());
        self.compile_statements();
        self.process("}".to_string());
        self.close_tag("whileStatement");
    }

    fn compile_expression(&mut self) {
        println!("inside expression");
        self.open_tag("expression");
        self.compile_term();

        while matches!(
            self.tokenizer.current_token.as_str(),
            "+" | "-" | "*" | "/" | "|" | "=" | "&lt;" | "&gt;" | "&amp;"
        ) {
            let op = self.tokenizer.current_token.clone();
            self.process(self.tokenizer.current_token.to_string());
            self.compile_term();
            match op.as_str() {
                "+" => self.vm_writer.write_arithmetic("add"),
                "*" => self.vm_writer.write_arithmetic("mul"),
                _ => println!("you passed in an op that did not match"),
            }
        }
        self.close_tag("expression");
    }

    fn compile_term(&mut self) {
        self.open_tag("term");
        match self.tokenizer.current_token_type {
            Some(TokenType::IntConst) => {
                println!(
                    "inside compile_term {}",
                    self.tokenizer.current_token.to_string()
                );
                let value: usize = self.tokenizer.current_token.parse().unwrap();
                self.vm_writer.write_push("constant", value);
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
                    _ => { /* nothing */ }
                }
            }
            Some(TokenType::Symbol) => match self.tokenizer.current_token.as_str() {
                "-" | "~" => {
                    self.process(self.tokenizer.current_token.to_string());
                    self.compile_term();
                }
                "(" => {
                    self.process("(".to_string());
                    self.compile_expression();
                    self.process(")".to_string());
                }
                _ => {
                    eprintln!(
                        "Syntax error: expected term, got '{}'",
                        self.tokenizer.current_token
                    );
                    std::process::exit(1);
                }
            },

            _ => print!("error"),
        }
        self.close_tag("term");
    }

    fn compile_expression_list(&mut self) -> i32 {
        self.open_tag("expressionList");
        let mut total = 0;
        if self.tokenizer.current_token != ")".to_string() {
            total += 1;
            self.compile_expression();
            while self.tokenizer.current_token == ','.to_string() {
                self.process(",".to_string());
                total += 1;
                self.compile_expression();
            }
        }
        self.close_tag("expressionList");
        total
    }

    fn compile_var_dec(&mut self) {
        self.open_tag("varDec");
        self.process("var".to_string());
        let var_type = self.tokenizer.current_token.clone();
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
        let var_name = self.tokenizer.current_token.clone();
        if self.tokenizer.current_token_type == Some(TokenType::Identifier) {
            self.sub_symbol_table
                .define(var_name, &var_type, symbol_table::SymbolType::Var);
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
                    let var_name = self.tokenizer.current_token.clone();
                    self.sub_symbol_table.define(
                        var_name,
                        &var_type,
                        symbol_table::SymbolType::Var,
                    );
                    self.process(self.tokenizer.current_token.to_string())
                }
                _ => {
                    eprintln!("Syntax error: got '{}'", self.tokenizer.current_token);
                    std::process::exit(1);
                }
            }
        }

        self.process(";".to_string());
        self.close_tag("varDec");
    }

    fn compile_parameter_list(&mut self) {
        if self.tokenizer.current_token != ")" {
            // Process type
            let var_type = self.tokenizer.current_token.clone();
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
            let var_name = self.tokenizer.current_token.clone();
            self.sub_symbol_table
                .define(var_name, &var_type, symbol_table::SymbolType::Arg);
            self.process(self.tokenizer.current_token.to_string());
            while self.tokenizer.current_token == "," {
                self.process(",".to_string());
                // Process type
                let var_type = self.tokenizer.current_token.clone();
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
                let var_name = self.tokenizer.current_token.clone();
                self.sub_symbol_table
                    .define(var_name, &var_type, symbol_table::SymbolType::Arg);
                self.process(self.tokenizer.current_token.to_string());
            }
        }
    }

    fn compile_class_var_dec(&mut self) {
        self.open_tag("classVarDec");
        let kind: symbol_table::SymbolType;
        let var_type: String;
        // static and field
        if self.tokenizer.current_token == "static" {
            kind = symbol_table::SymbolType::Static;
            self.process("static".to_string());
            // taking away this else if check mighht need to lookout for bigs on field
            // if self.tokenizer.current_token == "field"
        } else {
            kind = symbol_table::SymbolType::Field;
            self.process("field".to_string());
        }
        // type
        var_type = self.tokenizer.current_token.clone();

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
        let name = self.tokenizer.current_token.clone();
        self.symbol_table.define(name, &var_type, kind);
        self.process(self.tokenizer.current_token.clone());
        // identify multiple variables
        while self.tokenizer.current_token == "," {
            self.process(",".to_string());
            let name = self.tokenizer.current_token.clone();
            self.symbol_table.define(name, &var_type, kind);
            self.process(self.tokenizer.current_token.clone());
        }

        self.process(";".to_string());

        self.close_tag("classVarDec");
    }

    fn process(&mut self, token: String) {
        println!("current token: {}", self.tokenizer.current_token);
        if self.tokenizer.current_token == token {
            println!("check was correct");
            let cur_token = self.tokenizer.current_token.clone();
            let cur_type = self.tokenizer.current_token_type;
            // self.print_xml_token(&cur_token, cur_type);
            self.tokenizer.advance();
            println!("at the end {}", self.tokenizer.current_token);
        } else {
            eprintln!(
                "Syntax error: expected '{}', got '{}'",
                token, self.tokenizer.current_token
            );
            std::process::exit(1);
        }
    }

    fn write_line(&mut self, s: &str) {
        let pad = "  ".repeat(self.indent);
        // print!("you called write_line and we removed some stuff here")
        // writeln!(self.output, "{}{}", pad, s).unwrap();
    }

    fn open_tag(&mut self, tag: &str) {
        self.write_line(&format!("<{}>", tag));
        self.indent += 1;
    }

    fn close_tag(&mut self, tag: &str) {
        self.indent -= 1;
        self.write_line(&format!("</{}>", tag));
    }

    fn print_xml_token(&mut self, current_token: &String, current_token_type: Option<TokenType>) {
        match current_token_type {
            Some(TokenType::Identifier) => {
                // checks subroutine symbol table
                if let Some(kind) = self.sub_symbol_table.kind_of(current_token) {
                    let category = match kind {
                        symbol_table::SymbolType::Static => "static",
                        symbol_table::SymbolType::Field => "field",
                        symbol_table::SymbolType::Arg => "arg",
                        symbol_table::SymbolType::Var => "var",
                        _ => "class",
                    };
                    self.write_line(&format!(
                        "
                        <identifier> 
                        \t<name>{}</name>
                        \t<category>{}</category>
                        \t<index>{}</index> 
                        </identifier>
                    ",
                        current_token,
                        category,
                        self.sub_symbol_table.index_of(current_token)
                    ))
                    // if nothting is found we check the class symbol table
                } else if let Some(kind) = self.symbol_table.kind_of(current_token) {
                    let category = match kind {
                        symbol_table::SymbolType::Static => "static",
                        symbol_table::SymbolType::Field => "field",
                        symbol_table::SymbolType::Arg => "arg",
                        symbol_table::SymbolType::Var => "var",
                        _ => "class",
                    };
                    self.write_line(&format!(
                        "
                        <identifier> 
                        \t<name>{}</name>
                        \t<category>{}</category>
                        \t<index>{}</index> 
                        </identifier>
                    ",
                        current_token,
                        category,
                        self.symbol_table.index_of(current_token)
                    ))
                    // else nothing is found
                } else {
                    self.write_line(&format!("<identifier> {} </identifier>", current_token))
                }
            }
            Some(TokenType::Keyword) => {
                self.write_line(&format!("<keyword> {} </keyword>", current_token))
            }
            Some(TokenType::Symbol) => {
                self.write_line(&format!("<symbol> {} </symbol>", current_token))
            }
            Some(TokenType::IntConst) => self.write_line(&format!(
                "<integerConstant> {} </integerConstant>",
                current_token
            )),
            Some(TokenType::StringConst) => self.write_line(&format!(
                "<stringConstant> {} </stringConstant>",
                current_token
            )),
            _ => println!("no match"),
        }
    }
}
