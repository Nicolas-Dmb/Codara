use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ExtractionIssue {
    #[error("analysis warning: {0}")]
    Warning(AnalysisWarning),
    #[error("retryable issue: {0}")]
    Retryable(RetryableIssue),
    #[error("source code error: {0}")]
    SourceCodeError(SourceCodeIssue),
}

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
    #[error("ignored file: {path}")]
    IgnoredFile {
        path: String,
    },
}

#[derive(Debug, Error, PartialEq)]
pub enum SourceCodeIssue {
    #[error("invalid syntax in {path}: {reason}")]
    InvalidSyntax {
        path: String,
        reason: String,
    },
}