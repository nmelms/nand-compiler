use crate::compilation_engine::ComplationEngine;
use std::fs::File;
use std::io::Write;

pub struct VMWriter {
    output: File,
}

impl VMWriter {
    pub fn new(output: File) -> Self {
        Self { output }
    }

    pub fn write_function(&mut self, name: &str, n_vars: usize) {
        writeln!(self.output, "function {} {}", name, n_vars)
            .expect("error writing fucntion to file");
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        writeln!(self.output, "{}", command).expect("Failed to write arithmetic");
    }

    pub fn write_push(&mut self, segment: &str, index: usize) {
        writeln!(self.output, "push {} {}", segment, index).expect("Failed to write push");
    }

    pub fn write_call(&mut self, name: &str, n_args: i32) {
        writeln!(self.output, "call {} {}", name, n_args).expect("Failed to write call");
    }
    pub fn write_return(&mut self) {
        writeln!(self.output, "return").expect("Failed to write call");
    }

    pub fn write_pop(&mut self, segment: &str, index: usize) {
        writeln!(self.output, "pop {} {}", segment, index).expect("Failed to write push");
    }

    pub fn write_label(&mut self, label: &String){
        writeln!(self.output, "label L{}", label).expect("Failed to write label");
    }

    pub fn write_if(&mut self, label: &String){
        writeln!(self.output, "if-goto L{}", label).expect("Failed to write if");
    }

    pub fn write_goto(&mut self, label: &String){
        writeln!(self.output, "goto L{}", label).expect("Failed to write goto");
    }
}
