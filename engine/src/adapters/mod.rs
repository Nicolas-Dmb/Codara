mod python;

use crate::analysis::connector::Adapter;

use python::PythonAdapter;

pub fn default_adapters() -> Vec<Box<dyn Adapter>> {
    vec![
        Box::new(PythonAdapter::new()),
    ]
}