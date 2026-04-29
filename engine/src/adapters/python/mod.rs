use crate::analysis::connector::Adapter;
use crate::model::{AnalysisWarning, RawModule, RawSymbol, ExtractionIssue, SymbolKind};
use std::path::Path;
mod parser;
use parser::parse_code;
pub struct PythonAdapter {}

impl Adapter for PythonAdapter {

    fn supported_extensions(&self) -> &[&'static str] {
        &[".py"]
    }

    fn ignore_files(&self) -> &[&'static str] {
        &["__init__.py"]
    }

    fn extract(&self, url: &str)-> Result<RawModule, ExtractionIssue>{
        if self.should_ignore(url) {
            return Err(ExtractionIssue::Warning(AnalysisWarning::IgnoredFile{path:url.to_string()}));
        }
        let raw_module = RawModule::new(url.to_string());
        
        let ast_code = parse_code(Path::new(url))?;
        
        unimplemented!()
    }
}

fn extract_symbols(url: &str)-> Result<Vec<RawSymbol>, ExtractionIssue>{
    //let symbols = vec![];
    unimplemented!()
}
fn extract_class()->Result<Vec<RawSymbol>, ExtractionIssue>{
    unimplemented!()
}
fn extract_function()->Result<Vec<RawSymbol>, ExtractionIssue>{
    unimplemented!()
}
fn extract_methode()->Result<Vec<RawSymbol>, ExtractionIssue>{
    unimplemented!()
}