use std::fs::File;
use std::io::Write;
use crate::compilation_engine::ComplationEngine;

pub struct VMWriter{
    output: File,
}

impl VMWriter{
    pub fn new(output: File) -> Self{
        Self{ output }
    }

    pub fn write_function(&mut self,name: &str, n_vars: usize){
        writeln!(self.output, "function {} {}", name, n_vars).expect("error writing fucntion to file");
    }
}