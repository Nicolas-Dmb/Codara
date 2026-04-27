
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RelationId(String);

impl RelationId {
    pub fn new(
        module_id: ModuleId,
        kind: RelationKind,
        imported_name: String,
        source_path: String,
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
    Call,
    Extends,
    Implements,
    Uses,
}

impl RelationKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationKind::Import => "import",
            RelationKind::Call => "call",
            RelationKind::Extends => "extends",
            RelationKind::Implements => "implements",
            RelationKind::Uses => "uses",
        }
    }
}

#[derive(Debug)]
pub struct Relation{
    pub id: RelationId,
    pub run_id: RunId,
    pub module_id: ModuleId,
    pub imported_name: String,
    pub source_path: String,
    pub source_symbol_id: SymbolId,
    pub target_symbol_id: SymbolId,
    pub kind : RelationKind,
}

impl Relation {
    pub fn new(
        module_id: ModuleId,
        run_id: RunId,
        kind: RelationKind,
        imported_name: String,
        source_path: String,
        source_symbol_id: SymbolId,
        target_symbol_id: SymbolId,
        line: u32,
    ) -> Self {
        let id = RelationId::new(
            module_id,
            kind.clone(),
            imported_name.clone(),
            source_path.clone(),
            line,
        );
        Self {
            id,
            run_id,
            module_id,
            imported_name,
            source_path,
            source_symbol_id,
            target_symbol_id,
            kind,
        }
    }
}