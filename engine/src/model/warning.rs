
#[derive(Debug, PartialEq)]
pub enum RetryableIssue {
    UnreadableDirectory {
        path: String,
        reason: String,
    },
    UnreadableFile {
        path: String,
        reason: String,
    },
    AdapterFailed {
        path: String,
        reason: String,
    },
    UnresolvedImport {
        path: String,
        import_name: String,
    },
}

#[derive(Debug, PartialEq)]
pub enum AnalysisWarning {
    UnsupportedFileType {
        path: String,
    },
    UnsupportedSymbolKind {
        path: String,
        kind: String,
    },
    MissingSymbolName {
        path: String,
    },
    UnsupportedRelationKind {
        path: String,
        kind: String,
    },
}
