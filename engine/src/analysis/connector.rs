use crate::adapters::default_adapters;
use crate::model::error::{AnalysisWarning};
use crate::model::extractor::RawAnalysisResult;
pub struct AdapterRegistry {
    adapters: Vec<Box<dyn Adapter>>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        AdapterRegistry { adapters: default_adapters() }
    }

    fn find(&self, url: &str) -> Result<&dyn Adapter, AnalysisWarning> {
        self.adapters
            .iter()
            .find(|adapter| adapter.can_handle(url))
            .map(|adapter| adapter.as_ref())
            .ok_or_else(|| AnalysisWarning::UnsupportedFileType{path:url.to_string()})
    }

    pub fn find_and_extract(
        &self,
        url: &str,
    ) -> Result<RawAnalysisResult, AnalysisWarning> {
        let adapter = self.find(url)?;
        adapter.extract(url)
    }



}

pub trait Adapter {
    fn can_handle(&self, url: &str) -> bool;
    fn extract(&self, url: &str) -> Result<RawAnalysisResult, AnalysisWarning>;
}