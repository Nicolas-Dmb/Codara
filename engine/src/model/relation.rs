use std::fmt;
use super::module::ModuleId;
use super::run::RunId;
use super::symbol::{RawSymbolId, SymbolId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelationId(String);

impl RelationId {
    pub fn new(
        module_id: &ModuleId,
        kind: &RelationKind,
        imported_name: &str,
        source_path: &str,
        line: usize,
    ) -> Self {
        Self(format!(
            "{}::{}::{}::{}::{}",
            module_id,
            kind,
            source_path,
            imported_name,
            line
        ))
    }
}

impl fmt::Display for RelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelationKind {
    Import,
}

impl fmt::Display for RelationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelationKind::Import => f.write_str("import"),
        }
    }
}

#[derive(Debug)]
pub struct Relation {
    pub id: RelationId,
    pub run_id: RunId,
    pub module_id: ModuleId,
    pub imported_name: String,
    pub source_path: String,
    pub target_symbol_id: Option<SymbolId>,
    pub kind: RelationKind,
    pub line: usize,
}

impl Relation {
    pub fn new(
        module_id: ModuleId,
        run_id: RunId,
        kind: RelationKind,
        imported_name: String,
        source_path: String,
        target_symbol_id: Option<SymbolId>,
        line: usize,
    ) -> Self {
        let id = RelationId::new(
            &module_id,
            &kind,
            &imported_name,
            &source_path,
            line,
        );
        Self {
            id,
            run_id,
            module_id,
            imported_name,
            source_path,
            target_symbol_id,
            kind,
            line,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawRelationId(String);

impl RawRelationId {
    pub fn new(
        kind: &RelationKind,
        imported_name: &str,
        source_path: &str,
        line: usize,
    ) -> Self {
        Self(format!(
            "{}::{}::{}::{}",
            kind,
            source_path,
            imported_name,
            line
        ))
    }
}

impl fmt::Display for RawRelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawSymbolRelationId(String);

impl RawSymbolRelationId {
    pub fn new(
        module_source: String,
        source_path: String,
    ) -> Self {
        Self(format!(
            "{}::{}",
            module_source,
            source_path,
        ))
    }
}

impl fmt::Display for RawSymbolRelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RawRelation {
    pub id: RawRelationId,
    pub imported_name: String,
    pub source_path: String, /// Path of the file WHERE the import is declared (the origin of the relation),
    pub target_symbol_id: Option<RawSymbolId>, /// unimplemented for now
    pub kind: RelationKind,
    pub line: usize,
}

impl RawRelation {
    pub fn new(
        kind: RelationKind,
        imported_name: String,
        source_path: String,
        target_symbol_id: Option<RawSymbolId>,
        line: usize,
    ) -> Self {
        let id = RawRelationId::new(
            &kind,
            &imported_name,
            &source_path,
            line,
        );
        Self {
            id,
            imported_name,
            source_path,
            target_symbol_id,
            kind,
            line,
        }
    }

    pub fn into_relation(
        self,
        module_id: &ModuleId,
        run_id: &RunId,
    ) -> Relation {
        Relation::new(
            module_id.clone(),
            run_id.clone(),
            self.kind,
            self.imported_name,
            self.source_path,
            None,
            self.line,
        )
    }
}
