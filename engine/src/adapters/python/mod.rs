use crate::analysis::connector::Adapter;

pub struct PythonAdapter {
    supported_extensions: Vec<String>,
}

impl PythonAdapter {
    pub fn new() -> Self {
        PythonAdapter {
            supported_extensions: vec![".py".to_string()],
        }
    }
}

impl Adapter for PythonAdapter {

    fn can_handle(&self, url: &str) -> bool {
        self.supported_extensions
            .iter()
            .any(|ext| url.ends_with(ext))
    }

    fn extract(&self, url: &str) {}
}