use std::collections::HashMap;


pub struct SymbolTable {
    table: HashMap<String, Symbol>,
    static_index: usize,   
    field_index: usize,    
    arg_index: usize,      
    var_index: usize,  
}

pub struct Symbol{
    name: String,
    r#type: String, 
    kind: String,
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


}