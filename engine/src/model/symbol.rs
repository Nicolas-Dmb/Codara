use std::fmt;
use crate::model::{ModuleId, RawRelation, Relation, RelationId};

use super::run::RunId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolId(String);

impl SymbolId {
    pub fn new(module_id: &ModuleId, kind: &SymbolKind, name: &str, start_line: usize) -> Self {
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

#[derive(Debug, Clone)]
pub struct Symbol {
    pub id: SymbolId,
    pub run_id: RunId,
    pub module_id: ModuleId,
    pub parent_symbol_id: Option<SymbolId>,
    pub name: String,
    pub kind: SymbolKind,
    pub doc: String,
    pub location: String,
    pub start_line: usize,
    pub end_line: usize,
}

impl Symbol {
    pub fn new(
        module_id: &ModuleId,
        run_id: &RunId,
        parent_symbol_id: Option<&SymbolId>,
        kind: SymbolKind,
        name: String,
        doc: String,
        location: String,
        start_line: usize,
        end_line: usize,
    ) -> Self {
        let id = SymbolId::new(&module_id, &kind, &name, start_line);
        Self {
            id,
            run_id: run_id.clone(),
            module_id: module_id.clone(),
            parent_symbol_id: parent_symbol_id.cloned(),
            name,
            kind,
            doc,
            location,
            start_line,
            end_line,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawSymbolId(String);

impl RawSymbolId {
    pub fn new(kind: &SymbolKind, name: &str, start_line: usize) -> Self {
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

#[derive(Debug, PartialEq, Clone)]
pub struct RawSymbol {
    pub id: RawSymbolId,
    pub name: String,
    pub kind: SymbolKind,
    pub doc: String,
    pub location: String,
    pub children_symbols: Vec<RawSymbol>,
    pub children_relations: Vec<RawRelation>,
    pub start_line: usize,
    pub end_line: usize,
}

impl RawSymbol {
    pub fn new(
        name: String,
        kind: SymbolKind,
        doc: String,
        location: String,
        start_line: usize,
        end_line: usize,
    ) -> Self {
        let id = RawSymbolId::new(&kind, &name, start_line);
        Self {
            id,
            name,
            kind,
            doc,
            location,
            children_symbols: Vec::new(),
            children_relations: Vec::new(),
            start_line,
            end_line,
        }
    }

    pub fn add_children_symbol(&mut self, child: RawSymbol) {
        self.children_symbols.push(child);
    }

    pub fn add_children_relation(&mut self, relation: RawRelation) {
        self.children_relations.push(relation);
    }

    pub fn into_symbol(
        self,
        module_id: &ModuleId,
        run_id: &RunId,
        parent_symbol_id: Option<&SymbolId>,
    ) -> (Symbol, Vec<RawSymbol>, Vec<RawRelation>) {

        let symbol = Symbol::new(
            module_id,
            run_id,
            parent_symbol_id,
            self.kind,
            self.name,
            self.doc,
            self.location,
            self.start_line,
            self.end_line,
        );
        (symbol, self.children_symbols, self.children_relations)
    }
}
