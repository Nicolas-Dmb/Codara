use super::module::ModuleId;
use super::run::RunId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolId(String);

impl SymbolId {
    pub fn new(module_id: &ModuleId, kind: &SymbolKind, name: &str, start_line: u32) -> Self {
        Self(format!(
            "{}::{}::{}::{}",
            module_id.value(),
            kind.as_str(),
            name,
            start_line
        ))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    Class,
    Function,
    Method,
}

impl SymbolKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            SymbolKind::Class => "class",
            SymbolKind::Function => "function",
            SymbolKind::Method => "method",
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
