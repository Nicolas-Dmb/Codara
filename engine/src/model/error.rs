#[derive(Debug)]
pub enum ProjectError {
    MissingRepositoryName,
    MissingNamespace,
    InvalidRepositoryUrl,
}

#[derive(Debug)]
pub enum ModuleError {
    UnimplementedFileType,
    ParseFailed(String),
}

#[derive(Debug)]
pub enum SymbolError {
    UnimplementedKind(String),
    MissingName,
}

#[derive(Debug)]
pub enum RelationError {
    UnimplementedKind(String),
    UnresolvedImport(String),
}

#[derive(Debug)]
pub enum AnalysisWarning {
    Module {
        module_path: String,
        error: ModuleError,
    },
    Symbol {
        module_path: String,
        symbol_name: String,
        error: SymbolError,
    },
    Relation {
        module_path: String,
        error: RelationError,
    },
}
