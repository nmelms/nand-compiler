use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, Symbol>,
    static_index: usize,
    field_index: usize,
    arg_index: usize,
    var_index: usize,
}
#[derive(Debug)]
pub struct Symbol {
    symbol_type: String,
    kind: SymbolType,
    index: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum SymbolType {
    Static,
    Field,
    Arg,
    Var,
}

impl SymbolTable {
    pub fn new() -> Self {
        return Self {
            table: HashMap::new(),
            static_index: 0,
            field_index: 0,
            arg_index: 0,
            var_index: 0,
        };
    }

    pub fn reset(&mut self) {
        self.table = HashMap::new();
        self.static_index = 0;
        self.field_index = 0;
        self.arg_index = 0;
        self.var_index = 0;
    }

    pub fn define(&mut self, name: String, symbol_type: &String, kind: SymbolType) {
        let index = match kind {
            SymbolType::Static => {
                let temp = self.static_index;
                self.static_index += 1;
                temp
            }
            SymbolType::Field => {
                let temp = self.field_index;
                self.field_index += 1;
                temp
            }
            SymbolType::Arg => {
                let temp = self.arg_index;
                self.arg_index += 1;
                temp
            }
            SymbolType::Var => {
                let temp = self.var_index;
                self.var_index += 1;
                temp
            }
        };

        self.table.insert(
            name,
            Symbol {
                symbol_type: symbol_type.to_string(),
                kind: kind,
                index: index,
            },
        );

    }

    pub fn var_count(&self, kind: SymbolType) -> usize {
        match kind {
            SymbolType::Static => self.static_index,
            SymbolType::Field => self.field_index,
            SymbolType::Arg => self.arg_index,
            SymbolType::Var => self.var_index,
        }
    }

    pub fn kind_of(&self, name: &String) -> Option<SymbolType> {
        let value = self.table.get(name);

        match value {
            Some(symbol) => Some(symbol.kind),
            None => None,
        }
    }

    pub fn type_of(&self, name: &String) -> String {
        let value = self.table.get(name);

        match value {
            Some(symbol) => symbol.symbol_type.clone(),
            None => panic!("Error in type_of Symbol not found: {}", name),
        }
    }

    pub fn index_of(&self, name: &String) -> usize {
        let value = self.table.get(name);

        match value {
            Some(symbol) => symbol.index,
            None => panic!("Error in index_of Symbol not found: {}", name),
        }
    }
}
