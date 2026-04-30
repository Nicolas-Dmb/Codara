use std::fmt;
use crate::model::{Module, ModuleId, RawRelation, Relation};

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

#[derive(Debug)]
pub struct Symbol {
    pub id: SymbolId,
    pub run_id: RunId,
    pub module_id: ModuleId,
    pub name: String,
    pub kind: SymbolKind,
    pub doc: String,
    pub location: String,
    pub children_symbol: Vec<Symbol>,
    pub children_relation: Vec<Relation>,
    pub start_line: usize,
    pub end_line: usize,
}

impl Symbol {
    pub fn new(
        module_id: ModuleId,
        run_id: RunId,
        kind: SymbolKind,
        name: String,
        doc: String,
        location: String,
        children_symbol: Vec<Symbol>,
        children_relation: Vec<Relation>,
        start_line: usize,
        end_line: usize,
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
            children_symbol,
            children_relation,
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

#[derive(Debug, PartialEq)]
pub struct RawSymbol {
    pub id: RawSymbolId,
    pub name: String,
    pub kind: SymbolKind,
    pub doc: String,
    pub location: String,
    pub children_symbol: Vec<RawSymbol>,
    pub children_relation: Vec<RawRelation>,
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
            children_symbol: Vec::new(),
            children_relation: Vec::new(),
            start_line,
            end_line,
        }
    }

    pub fn add_children_symbol(&mut self, child: RawSymbol) {
        self.children_symbol.push(child);
    }

    pub fn add_children_relation(&mut self, relation: RawRelation) {
        self.children_relation.push(relation);
    }

    pub fn into_symbol(
        self,
        module_id: &ModuleId,
        run_id: &RunId,
    ) -> Symbol {
        Symbol::new(
            module_id.clone(),
            run_id.clone(),
            self.kind,
            self.name,
            self.doc,
            self.location,
            self.children_symbol.into_iter().map(|s| s.into_symbol(module_id, run_id)).collect(),
            self.children_relation.into_iter().map(|r| r.into_relation(module_id, run_id)).collect(),
            self.start_line,
            self.end_line,
        )
    }
}
