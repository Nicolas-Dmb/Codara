use crate::analysis::connector::Adapter;
use crate::model::{AnalysisWarning, ExtractionIssue, RawModule, RelationKind, RawRelation, SourceCodeIssue, RetryableIssue};
mod parser;
    use parser::parse_code;
mod symbol_extractor;
use crate::adapters::python::symbol_extractor::{extract_name_from_symbol};
mod module_extractor;
use crate::adapters::python::module_extractor::extractor;

/// Python adapter for `.py` files.                                                                                                          
///                                                                                                                                         
/// Supported SymbolKinds: Class, Function, Method                                                                                           
/// Supported RelationKinds: Import                                                                                                          
/// Ignored files: `__init__.py`   

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
        let mut raw_module = RawModule::new(url.to_string());
        
        let source_code = self.read_source_code(url)?;

        let ast_code = parse_code(&source_code);
        
        let items = extractor(&ast_code, &source_code, url)?;

        items.symbols.into_iter().for_each(|symbol| raw_module.add_symbol(symbol));
        items.relations.into_iter().for_each(|relation| raw_module.add_relation(relation));

        Ok(raw_module)
    }
}

#[derive(Clone, PartialEq)]
enum Scope {
    Module,
    Class,
    Function,
}


fn extract_import(node: tree_sitter::Node, source_code: &str, path: &str) -> Result<Vec<RawRelation>, ExtractionIssue> {
    let imported_names = extract_name_from_import(node, source_code)?;

    let relations: Vec<RawRelation> = imported_names.into_iter().map(|imported_name| {
        RawRelation::new(
            RelationKind::Import,
            imported_name,
            path.to_string(),
        None,
        node.start_position().row + 1,
        )
    }).collect();
    Ok(relations)
}

fn extract_name_from_import(node: tree_sitter::Node, source_code: &str) -> Result<Vec<String>, ExtractionIssue> {
    let mut cursor = node.walk();
    let name_nodes: Vec<String> = node.children_by_field_name("name", &mut cursor)
        .map(|n| n.utf8_text(source_code.as_bytes())
            .expect("source is &str, always valid UTF-8")
            .to_string())
        .collect();
    
    let statement_name_node = node.child_by_field_name("module_name");

    let statement_name = match statement_name_node {
        Some(node) => node
            .utf8_text(source_code.as_bytes())
            .expect("source is &str, always valid UTF-8")
            .to_string(),
        None => "".to_string(),
    };

    Ok(name_nodes.iter().map(|n| if statement_name.is_empty() {
        n.to_string()
    } else {
        format!("{}/{}", statement_name, n)
    }).collect())
}


#[cfg(test)]
mod tests {
    use super::*;

    adapter_contract_tests!(
        adapter: PythonAdapter {},
        extension: ".py",
        valid_source: "import os\n\nclass Foo:\n    \"\"\"Foo doc\"\"\"\n    def bar(self):\n        pass\n\ndef baz():\n    \"\"\"baz doc\"\"\"\n    return 1\n",
        ignored_filename: "__init__.py",
    );
}

#[cfg(test)]
mod python_specific_tests {
    use crate::model::SymbolKind;

    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn adapter() -> PythonAdapter {
        PythonAdapter {}
    }

    #[test]
    fn test_extractor_propagates_error() {
        let source = "x = 1";
        let tree = parse_code(source);
        let root = tree.root_node();

        let result = extract_name_from_symbol(root, source, "test.py");
        assert_eq!(
            result.unwrap_err(),
            ExtractionIssue::SourceCodeError(SourceCodeIssue::InvalidSyntax {
                path: "test.py".to_string(),
                reason: "Missing name in line '1'".to_string(),
            })
        );
    }

    #[test]
    fn test_extract_returns_raw_module() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("example.py");
        let mut file = File::create(&file_path).unwrap();
        write!(file, r#"class Foo:
    """Foo doc"""
    def bar(self):
        pass

def baz():
    """baz doc"""
    return 1
"#).unwrap();

        let url = file_path.to_string_lossy().to_string();
        let module = adapter().extract(&url).unwrap();

        assert_eq!(module.relative_path, url);
        assert_eq!(module.name, "example.py");
        assert_eq!(module.symbols.len(), 2);

        let class = &module.symbols[0];
        assert_eq!(class.name, "Foo");
        assert_eq!(class.kind, SymbolKind::Class);
        assert_eq!(class.children_symbols.len(), 1);

        let method = &class.children_symbols[0];
        assert_eq!(method.name, "bar");
        assert_eq!(method.kind, SymbolKind::Method);

        let func = &module.symbols[1];
        assert_eq!(func.name, "baz");
        assert_eq!(func.kind, SymbolKind::Function);
    }
}
