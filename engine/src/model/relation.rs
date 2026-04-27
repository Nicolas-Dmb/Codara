use super::module::ModuleId;
use super::run::RunId;
use super::symbol::SymbolId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelationId(String);

impl RelationId {
    pub fn new(
        module_id: &ModuleId,
        kind: &RelationKind,
        imported_name: &str,
        source_path: &str,
        line: u32,
    ) -> Self {
        Self(format!(
            "{}::{}::{}::{}::{}",
            module_id.value(),
            kind.as_str(),
            source_path,
            imported_name,
            line
        ))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelationKind {
    Import,
}

impl RelationKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationKind::Import => "import",
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
    pub line: u32,
}

impl Relation {
    pub fn new(
        module_id: ModuleId,
        run_id: RunId,
        kind: RelationKind,
        imported_name: String,
        source_path: String,
        target_symbol_id: Option<SymbolId>,
        line: u32,
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
