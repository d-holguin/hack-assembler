use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, u16>,
    next_variable_address: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        let table = Self::map_with_predefined_symbols();

        SymbolTable {
            table,
            next_variable_address: 16,
        }
    }
    pub fn add_variable(&mut self, symbol: String) -> u16 {
        if !self.table.contains_key(&symbol) {
            let address = self.next_variable_address;
            self.table.insert(symbol, address);
            self.next_variable_address += 1;
            address
        } else {
            *self.table.get(&symbol).unwrap()
        }
    }

    pub fn add_label(&mut self, symbol: String, symbol_address: u16) {
        self.table.entry(symbol).or_insert(symbol_address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> Option<&u16> {
        self.table.get(symbol)
    }

    fn map_with_predefined_symbols() -> HashMap<String, u16> {
        let mut table = HashMap::new();
        let symbols = [("SP", 0), ("LCL", 1), ("ARG", 2), ("THIS", 3), ("THAT", 4)];
        for (symbol, address) in symbols.iter() {
            table.insert(symbol.to_string(), *address);
        }
        for i in 0..16 {
            table.insert(format!("R{}", i), i);
        }
        table.insert("SCREEN".to_string(), 16384);
        table.insert("KBD".to_string(), 24576);

        table
    }
}
