use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum RetryableIssue {
    #[error("unreadable directory {path}: {reason}")]
    UnreadableDirectory {
        path: String,
        reason: String,
    },
    #[error("unreadable file {path}: {reason}")]
    UnreadableFile {
        path: String,
        reason: String,
    },
    #[error("adapter failed for {path}: {reason}")]
    AdapterFailed {
        path: String,
        reason: String,
    },
    #[error("unresolved import '{import_name}' in {path}")]
    UnresolvedImport {
        path: String,
        import_name: String,
    },
}

#[derive(Debug, Error, PartialEq)]
pub enum AnalysisWarning {
    #[error("unsupported file type: {path}")]
    UnsupportedFileType {
        path: String,
    },
    #[error("unsupported symbol kind '{kind}' in {path}")]
    UnsupportedSymbolKind {
        path: String,
        kind: String,
    },
    #[error("missing symbol name in {path}")]
    MissingSymbolName {
        path: String,
    },
    #[error("unsupported relation kind '{kind}' in {path}")]
    UnsupportedRelationKind {
        path: String,
        kind: String,
    },
}
