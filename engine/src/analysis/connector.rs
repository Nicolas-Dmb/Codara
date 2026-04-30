use crate::adapters::default_adapters;
use crate::model::{AnalysisWarning, RawModule, ExtractionIssue,RetryableIssue};

pub trait AdapterRegistryTrait {
    fn find_and_extract(&self, url: &str) -> Result<RawModule, ExtractionIssue>;
}
pub struct AdapterRegistry {
    adapters: Vec<Box<dyn Adapter>>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        AdapterRegistry { adapters: default_adapters() }
    }

    fn find(&self, url: &str) -> Result<&dyn Adapter, ExtractionIssue> {
        self.adapters
            .iter()
            .find(|adapter| adapter.can_handle(url))
            .map(|adapter| adapter.as_ref())
            .ok_or_else(|| ExtractionIssue::Warning(AnalysisWarning::UnsupportedFileType{path:url.to_string()}))
    }
}

impl AdapterRegistryTrait for AdapterRegistry {
    fn find_and_extract(
        &self,
        url: &str,
    ) -> Result<RawModule, ExtractionIssue> {
        let adapter = self.find(url)?;
        adapter.extract(url)
    }
}

pub trait Adapter {
    fn supported_extensions(&self) -> &[&'static str];

    fn can_handle(&self, url: &str) -> bool {
        self.supported_extensions()
            .iter()
            .any(|ext| url.ends_with(ext))
    }

    fn ignore_files(&self) -> &[&'static str];

    fn should_ignore(&self, url: &str) -> bool {
        self.ignore_files().iter().any(|ignore| url.ends_with(ignore))
    }

    fn extract(&self, url: &str) -> Result<RawModule, ExtractionIssue>;

    fn read_source_code(&self, url: &str) -> Result<String, ExtractionIssue> {
        let source_code = std::fs::read_to_string(url).map_err(|err| {
            ExtractionIssue::Retryable(RetryableIssue::UnreadableFile {
                    path: url.to_string(),
                    reason: err.to_string(),
                })
            }
        )?;
        Ok(source_code)
    }
}

#[cfg(test)]
mod adapter_registry_tests {
    use super::*;

    struct FakeAdapter;
    impl Adapter for FakeAdapter {
        fn supported_extensions(&self) -> &[&'static str] {
            &[".fake"]
        }

        fn extract(&self, url: &str) -> Result<RawModule, ExtractionIssue> {
            Ok(RawModule::new(url.to_string()))
        }

        fn ignore_files(&self) -> &[&'static str] {
            &[]
        }
    }

    #[test]
    fn test_adapter_registry() {
        let registry = AdapterRegistry {
            adapters: vec![Box::new(FakeAdapter)],
        };

        let result = registry.find_and_extract("test.fake");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().relative_path, "test.fake");

        let result = registry.find_and_extract("test.unknown");
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            ExtractionIssue::Warning(AnalysisWarning::UnsupportedFileType{path:"test.unknown".to_string()})
        );
    }

}
