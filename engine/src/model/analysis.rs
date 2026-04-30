use crate::model::module::RawModule;
use crate::model::warning::{AnalysisWarning,RetryableIssue,SourceCodeIssue};
use crate::model::{RawSymbol, RawRelation};

#[derive(Debug, Default, PartialEq)]
pub struct AnalysisReport {
    pub retryables: Vec<RetryableIssue>,
    pub warnings: Vec<AnalysisWarning>,
    pub source_code_issues: Vec<SourceCodeIssue>,
    pub raw_modules: Vec<RawModule>,
}

impl AnalysisReport {
    pub fn new() -> Self {
        Self {
            retryables: Vec::new(),
            warnings: Vec::new(),
            source_code_issues: Vec::new(),
            raw_modules: Vec::new(),
        }
    }

    pub fn add_retryable(&mut self, issue: RetryableIssue) {
        self.retryables.push(issue);
    }

    pub fn add_warning(&mut self, warning: AnalysisWarning) {
        self.warnings.push(warning);
    }

    pub fn add_source_code_issue(&mut self, issue: SourceCodeIssue) {
        self.source_code_issues.push(issue);
    }

    pub fn merge(&mut self, other: AnalysisReport) {
        self.retryables.extend(other.retryables);
        self.warnings.extend(other.warnings);
        self.source_code_issues.extend(other.source_code_issues);
        self.raw_modules.extend(other.raw_modules);
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

    pub fn add_module(&mut self, raw_module: RawModule) {
        self.raw_modules.push(raw_module);
    }

    pub fn has_data(&self) -> bool {
        !self.raw_modules.is_empty()
        || self.has_retryables()
        || self.has_warnings()
    }
}


pub struct ExtractedItems{
    pub symbols: Vec<RawSymbol>,
    pub relations: Vec<RawRelation>,
}