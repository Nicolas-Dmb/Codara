use std::fmt;
use super::module::ModuleId;
use super::run::RunId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolId(String);

impl SymbolId {
    pub fn new(module_id: &ModuleId, kind: &SymbolKind, name: &str, start_line: u32) -> Self {
        Self(format!(
            "{}::{}::{}::{}",
            module_id,
            kind,
            name,
            start_line
        ))
    }
}

impl fmt::Display for SymbolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    Class,
    Function,
    Method,
}

impl fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolKind::Class => f.write_str("class"),
            SymbolKind::Function => f.write_str("function"),
            SymbolKind::Method => f.write_str("method"),
        }
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub id: SymbolId,
    pub run_id: RunId,
    pub module_id: ModuleId,
    pub name: String,
    pub kind: SymbolKind,
    pub doc: String,
    pub location: String,
    pub parent_symbol_id: Option<SymbolId>,
    pub start_line: u32,
    pub end_line: u32,
}

impl Symbol {
    pub fn new(
        module_id: ModuleId,
        run_id: RunId,
        kind: SymbolKind,
        name: String,
        doc: String,
        location: String,
        parent_symbol_id: Option<SymbolId>,
        start_line: u32,
        end_line: u32,
    ) -> Self {
        let id = SymbolId::new(&module_id, &kind, &name, start_line);
        Self {
            id,
            run_id,
            module_id,
            name,
            kind,
            doc,
            location,
            parent_symbol_id,
            start_line,
            end_line,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawSymbolId(String);

impl RawSymbolId {
    pub fn new(kind: &SymbolKind, name: &str, start_line: u32) -> Self {
        Self(format!(
            "{}::{}::{}",
            kind,
            name,
            start_line
        ))
    }
}

impl fmt::Display for RawSymbolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, PartialEq)]
pub struct RawSymbol {
    pub id: RawSymbolId,
    pub name: String,
    pub kind: SymbolKind,
    pub doc: String,
    pub location: String,
    pub children_symbol: Vec<RawSymbol>,
    pub start_line: u32,
    pub end_line: u32,
}

impl RawSymbol {
    pub fn new(
        name: String,
        kind: SymbolKind,
        doc: String,
        location: String,
        start_line: u32,
        end_line: u32,
    ) -> Self {
        let id = RawSymbolId::new(&kind, &name, start_line);
        Self {
            id,
            name,
            kind,
            doc,
            location,
            children_symbol: Vec::new(),
            start_line,
            end_line,
        }
    }

    pub fn add_child(&mut self, child: RawSymbol) {
        self.children_symbol.push(child);
    }

    pub fn into_symbol(
        self,
        module_id: ModuleId,
        run_id: RunId,
        parent_symbol_id: Option<SymbolId>,
    ) -> (Symbol, Vec<RawSymbol>) {
        let symbol = Symbol::new(
            module_id,
            run_id,
            self.kind,
            self.name,
            self.doc,
            self.location,
            parent_symbol_id,
            self.start_line,
            self.end_line,
        );
        (symbol, self.children_symbol)
    }
}
