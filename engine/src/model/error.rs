#[derive(Debug, PartialEq)]
pub enum ProjectError {
    MissingRepositoryName,
    MissingNamespace,
    InvalidRepositoryUrl,
}

#[derive(Debug, PartialEq)]
pub enum RunError {
    EmptyRepository,
    ReadRepositoryFailed(String),
    CloneFailed(String),
}

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
        extension: Option<String>,
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

#[derive(Debug, Default, PartialEq)]
pub struct AnalysisReport {
    pub retryables: Vec<RetryableIssue>,
    pub warnings: Vec<AnalysisWarning>,
}

impl AnalysisReport {
    pub fn new() -> Self {
        Self {
            retryables: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_retryable(&mut self, issue: RetryableIssue) {
        self.retryables.push(issue);
    }

    pub fn add_warning(&mut self, warning: AnalysisWarning) {
        self.warnings.push(warning);
    }

    pub fn merge(&mut self, other: AnalysisReport) {
        self.retryables.extend(other.retryables);
        self.warnings.extend(other.warnings);
    }

    pub fn has_retryables(&self) -> bool {
        !self.retryables.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn is_clean(&self) -> bool {
        self.retryables.is_empty() && self.warnings.is_empty()
    }
}