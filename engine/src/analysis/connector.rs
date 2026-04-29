use crate::adapters::default_adapters;
use crate::model::warning::AnalysisWarning;
use crate::model::module::{RawModule};

pub trait AdapterRegistryTrait {
    fn find_and_extract(&self, url: &str) -> Result<RawModule, AnalysisWarning>;
}
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
}

impl AdapterRegistryTrait for AdapterRegistry {
    fn find_and_extract(
        &self,
        url: &str,
    ) -> Result<RawModule, AnalysisWarning> {
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
    
    fn extract(&self, url: &str) -> Result<RawModule, AnalysisWarning>;
}

#[cfg(test)]
mod adapter_registry_tests {
    use super::*;

    struct FakeAdapter;
    impl Adapter for FakeAdapter {
        fn supported_extensions(&self) -> &[&'static str] {
            &[".fake"]
        }

        fn extract(&self, url: &str) -> Result<RawModule, AnalysisWarning> {
            Ok(RawModule::new(url.to_string(), "fake_module".to_string()))
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
            AnalysisWarning::UnsupportedFileType{path:"test.unknown".to_string()}
        );
    }

}