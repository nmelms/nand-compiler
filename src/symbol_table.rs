use std::collections::HashMap;


pub struct SymbolTable {
    table: HashMap<String, Symbol>,
    static_index: usize,   
    field_index: usize,    
    arg_index: usize,      
    var_index: usize,  
}

pub struct Symbol{
    symbol_type: String, 
    kind: SymbolType,
    index: usize,
}

pub enum SymbolType{
    Static,
    Field, 
    Arg,
    Var,
}


impl SymbolTable{
    pub fn new() -> Self{
    return Self{ 
        table: HashMap::new(), 
        static_index: 0, 
        field_index: 0, 
        arg_index: 0,
        var_index: 0 
    };
    }

    pub fn reset(&mut self){
        self.table = HashMap::new(); 
        self.static_index = 0;
        self.field_index =  0;
        self.arg_index = 0;
        self.var_index = 0;
    }

    pub fn define(&mut self, name:String, symbol_type: String, kind: SymbolType){
        let index = match kind{
            SymbolType::Static => {
                let temp = self.static_index;
                self.static_index += 1;
                temp
            },
            SymbolType::Field => {
                let temp = self.field_index;
                self.field_index += 1;
                temp
            },
            SymbolType::Arg => {
                let temp = self.arg_index;
                self.arg_index += 1;
                temp
            },
            SymbolType::Var => {
                let temp = self.var_index;
                self.var_index += 1;
                temp
            },
        };

        self.table.insert(name,  Symbol { 
            symbol_type: symbol_type, 
            kind: kind, 
            index: index
        });
    }


} 