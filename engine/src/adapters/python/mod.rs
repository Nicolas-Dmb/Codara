use std::fmt;
use crate::analysis::connector::Adapter;
use crate::model::{AnalysisWarning, ExtractionIssue, RawModule, RawSymbol, RelationKind, SymbolKind, SourceCodeIssue, RawRelation, ExtractedItems};
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

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scope::Module => f.write_str("module"),
            Scope::Class => f.write_str("class"),
            Scope::Function => f.write_str("function"),
        }
    }
}

fn extractor( tree: &tree_sitter::Tree, source_code: &str, path: &str) 
    -> Result<ExtractedItems, ExtractionIssue> {
    let mut items = ExtractedItems {
        symbols: Vec::new(),
        relations: Vec::new(),
    };
    let root = tree.root_node();

    visit_node(root, Scope::Module, source_code, &mut items, path)?;

    Ok(items)
}

fn visit_node(node: tree_sitter::Node,scope:Scope, source_code: &str, items: &mut ExtractedItems, path: &str)-> Result<(), ExtractionIssue> {
    match node.kind() {
        "class_definition" => {
            items.symbols.push(extract_class(node, source_code, path)?);
        }
        "function_definition" => {
            items.symbols.push(extract_function(node, scope.clone(), source_code, path)?);
        }
        "import_statement" | "import_from_statement" => {
            items.relations.extend(extract_import(node, source_code, path)?);
        }
        _ => {
            for child in node.children(&mut node.walk()) {
                visit_node(child, scope.clone(), source_code, items, path)?;
            }
        }
    }
    Ok(())
}

fn extract_class(node: tree_sitter::Node, source_code: &str, path: &str)-> Result<RawSymbol, ExtractionIssue>{
    let class_name = extract_name_from_symbol(node, source_code, path)?;
    let class_doc = extract_docstring(node, source_code);
    
    let mut symbol = RawSymbol::new(
        class_name,
        SymbolKind::Class,
        class_doc,
        node.start_position().row + 1,
        node.end_position().row + 1,
    );

    extract_symbol_body(node, source_code, path, &mut symbol, Scope::Class)?;

    Ok(symbol)
}

fn extract_function(node: tree_sitter::Node, scope:Scope, source_code: &str, path: &str)->Result<RawSymbol, ExtractionIssue>{
    let function_name = extract_name_from_symbol(node, source_code, path)?;
    let function_doc = extract_docstring(node, source_code);
    let kind = if scope == Scope::Class {
        SymbolKind::Method
    } else {
        SymbolKind::Function
    };
    let mut function_symbol = RawSymbol::new(
        function_name,
        kind,
        function_doc,
        node.start_position().row + 1,
        node.end_position().row + 1,
    );

    extract_symbol_body(node, source_code, path,&mut function_symbol, Scope::Function)?;

    Ok(function_symbol)
}

fn extract_symbol_body(node: tree_sitter::Node, source_code: &str, path: &str, symbol: &mut RawSymbol, scope: Scope) -> Result<(), ExtractionIssue> {
    let mut children = ExtractedItems {
        symbols: Vec::new(),
        relations: Vec::new(),
    };

    let child_node = node.child_by_field_name("body")
        .ok_or_else(|| ExtractionIssue::SourceCodeError(SourceCodeIssue::InvalidSyntax{
            path: path.to_string(),
            reason: format!("Missing body for {} in line '{}'", scope, node.start_position().row + 1),
        }))?;
    visit_node(child_node, scope, source_code,  &mut children, path)?;

    children.symbols
    .into_iter()
    .for_each(|child| symbol.add_children_symbol(child));
    children.relations
    .into_iter()
    .for_each(|relation| symbol.add_children_relation(relation));

    Ok(())
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

fn extract_docstring(node: tree_sitter::Node, source_code: &str) -> String {
    if let Some(body) = node.child_by_field_name("body") {
        let mut cursor = body.walk();

        if let Some(first_child) = body.children(&mut cursor).next() {
            if first_child.kind() == "expression_statement" {
                if let Some(string_node) = first_child.child(0) {
                    if string_node.kind() == "string" {
                        return string_node
                            .utf8_text(source_code.as_bytes())
                            .expect("source is &str, always valid UTF-8")
                            .to_string();
                    }
                }
            }
        }
    }

    String::new()
}

fn extract_name_from_symbol(node: tree_sitter::Node, source_code: &str, path: &str) -> Result<String, ExtractionIssue> {
    let name_node = node.child_by_field_name("name").ok_or_else(||
        ExtractionIssue::SourceCodeError(SourceCodeIssue::InvalidSyntax{
            path: path.to_string(),
            reason: format!("Missing name in line '{}'", node.start_position().row + 1),
        })
    )?;

    Ok(name_node
        .utf8_text(source_code.as_bytes())
        .expect("source is &str, always valid UTF-8")
        .to_string())
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
        format!("{}::{}", statement_name, n)
    }).collect())
}

#[cfg(test)]
mod extract_tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn adapter() -> PythonAdapter {
        PythonAdapter {}
    }

    #[test]
    fn test_extract_ignored_file() {
        let result = adapter().extract("some/path/__init__.py");
        assert_eq!(
            result.unwrap_err(),
            ExtractionIssue::Warning(AnalysisWarning::IgnoredFile {
                path: "some/path/__init__.py".to_string(),
            })
        );
    }

    #[test]
    fn test_extract_unreadable_file() {
        let result = adapter().extract("/nonexistent/path/file.py");
        assert!(matches!(
            result.unwrap_err(),
            ExtractionIssue::Retryable(RetryableIssue::UnreadableFile { .. })
        ));
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
        assert_eq!(class.children_symbol.len(), 1);

        let method = &class.children_symbol[0];
        assert_eq!(method.name, "bar");
        assert_eq!(method.kind, SymbolKind::Method);

        let func = &module.symbols[1];
        assert_eq!(func.name, "baz");
        assert_eq!(func.kind, SymbolKind::Function);
    }
}

#[cfg(test)]
mod extractor_tests {
    use super::*;

    #[test]
    fn test_extractor_returns_symbols() {
        let source = r#"class Foo:
    """Foo doc"""
    def bar(self):
        pass

def baz():
    return 1
"#;
        let tree = parse_code(source);
        let items = extractor(&tree, source, "test.py").unwrap();

        assert_eq!(items.symbols.len(), 2);

        assert_eq!(items.symbols[0].name, "Foo");
        assert_eq!(items.symbols[0].kind, SymbolKind::Class);
        assert_eq!(items.symbols[0].children_symbol.len(), 1);
        assert_eq!(items.symbols[0].children_symbol[0].name, "bar");
        assert_eq!(items.symbols[0].children_symbol[0].kind, SymbolKind::Method);

        assert_eq!(items.symbols[1].name, "baz");
        assert_eq!(items.symbols[1].kind, SymbolKind::Function);
    }

}

#[cfg(test)]
mod visit_node_tests {
    use super::*;

    fn get_first_child_of_kind<'a>(node: tree_sitter::Node<'a>, kind: &str) -> tree_sitter::Node<'a> {
        let mut cursor = node.walk();
        node.children(&mut cursor)
            .find(|c| c.kind() == kind)
            .unwrap_or_else(|| panic!("no child of kind '{kind}' found"))
    }

    #[test]
    fn test_class_definition_success() {
        let source = "class Foo:\n    pass\n";
        let tree = parse_code(source);
        let class_node = get_first_child_of_kind(tree.root_node(), "class_definition");

        let mut items = ExtractedItems { symbols: Vec::new(), relations: Vec::new() };
        visit_node(class_node, Scope::Module, source, &mut items, "test.py").unwrap();

        assert_eq!(items.symbols.len(), 1);
        assert_eq!(items.symbols[0].name, "Foo");
        assert_eq!(items.symbols[0].kind, SymbolKind::Class);
    }

    #[test]
    fn test_class_definition_error() {
        let source = "x = 1";
        let tree = parse_code(source);

        let result = extract_class(tree.root_node(), source, "test.py");
        assert!(matches!(
            result.unwrap_err(),
            ExtractionIssue::SourceCodeError(SourceCodeIssue::InvalidSyntax { .. })
        ));
    }

    #[test]
    fn test_function_definition_success() {
        let source = "def bar():\n    pass\n";
        let tree = parse_code(source);
        let func_node = get_first_child_of_kind(tree.root_node(), "function_definition");

        let mut items = ExtractedItems { symbols: Vec::new(), relations: Vec::new() };
        visit_node(func_node, Scope::Module, source, &mut items, "test.py").unwrap();

        assert_eq!(items.symbols.len(), 1);
        assert_eq!(items.symbols[0].name, "bar");
        assert_eq!(items.symbols[0].kind, SymbolKind::Function);
    }

    #[test]
    fn test_function_definition_error() {
        let source = "x = 1";
        let tree = parse_code(source);

        let result = extract_function(tree.root_node(), Scope::Module, source, "test.py");
        assert!(matches!(
            result.unwrap_err(),
            ExtractionIssue::SourceCodeError(SourceCodeIssue::InvalidSyntax { .. })
        ));
    }

    #[test]
    fn test_children_recursion_success() {
        let source = "def foo():\n    pass\n\nclass Bar:\n    pass\n";
        let tree = parse_code(source);

        let mut items = ExtractedItems { symbols: Vec::new(), relations: Vec::new() };
        visit_node(tree.root_node(), Scope::Module, source, &mut items, "test.py").unwrap();

        assert_eq!(items.symbols.len(), 2);
        assert_eq!(items.symbols[0].name, "foo");
        assert_eq!(items.symbols[0].kind, SymbolKind::Function);
        assert_eq!(items.symbols[1].name, "Bar");
        assert_eq!(items.symbols[1].kind, SymbolKind::Class);
    }

    #[test]
    fn test_children_recursion_stops_on_first_error() {
        let source = "def foo():\n    pass\n";
        let tree = parse_code(source);

        let mut items = ExtractedItems { symbols: Vec::new(), relations: Vec::new() };
        visit_node(tree.root_node(), Scope::Module, source, &mut items, "test.py").unwrap();
        assert_eq!(items.symbols.len(), 1);

        let result = extract_class(tree.root_node(), source, "test.py");
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod extract_class_tests {
    use super::*;

    fn find_node_by_kind<'a>(node: tree_sitter::Node<'a>, kind: &str) -> Option<tree_sitter::Node<'a>> {
        if node.kind() == kind {
            return Some(node);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if let Some(found) = find_node_by_kind(child, kind) {
                return Some(found);
            }
        }
        None
    }

    #[test]
    fn test_name_error() {
        let source = "x = 1";
        let tree = parse_code(source);

        let result = extract_class(tree.root_node(), source, "test.py");
        assert_eq!(
            result.unwrap_err(),
            ExtractionIssue::SourceCodeError(SourceCodeIssue::InvalidSyntax {
                path: "test.py".to_string(),
                reason: "Missing name in line '1'".to_string(),
            })
        );
    }

    #[test]
    fn test_body_error() {
        let source = "foo(bar=1)";
        let tree = parse_code(source);
        let kwarg = find_node_by_kind(tree.root_node(), "keyword_argument").unwrap();

        let result = extract_class(kwarg, source, "test.py");
        assert_eq!(
            result.unwrap_err(),
            ExtractionIssue::SourceCodeError(SourceCodeIssue::InvalidSyntax {
                path: "test.py".to_string(),
                reason: "Missing body for class in line '1'".to_string(),
            })
        );
    }

    #[test]
    fn test_success() {
        let source = "class Foo:\n    \"\"\"Foo doc\"\"\"\n    def bar(self):\n        pass\n";
        let tree = parse_code(source);
        let class_node = find_node_by_kind(tree.root_node(), "class_definition").unwrap();

        let symbol = extract_class(class_node, source, "test.py").unwrap();

        assert_eq!(symbol.name, "Foo");
        assert_eq!(symbol.kind, SymbolKind::Class);
        assert_eq!(symbol.doc, "\"\"\"Foo doc\"\"\"");
        assert_eq!(symbol.start_line, 1);
        assert_eq!(symbol.end_line, 4);
        assert_eq!(symbol.children_symbol.len(), 1);
        assert_eq!(symbol.children_symbol[0].name, "bar");
        assert_eq!(symbol.children_symbol[0].kind, SymbolKind::Method);
    }
}

#[cfg(test)]
mod extract_function_tests {
    use super::*;

    fn find_node_by_kind<'a>(node: tree_sitter::Node<'a>, kind: &str) -> Option<tree_sitter::Node<'a>> {
        if node.kind() == kind {
            return Some(node);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if let Some(found) = find_node_by_kind(child, kind) {
                return Some(found);
            }
        }
        None
    }

    #[test]
    fn test_name_error() {
        let source = "x = 1";
        let tree = parse_code(source);

        let result = extract_function(tree.root_node(), Scope::Module, source, "test.py");
        assert_eq!(
            result.unwrap_err(),
            ExtractionIssue::SourceCodeError(SourceCodeIssue::InvalidSyntax {
                path: "test.py".to_string(),
                reason: "Missing name in line '1'".to_string(),
            })
        );
    }

    #[test]
    fn test_success_function() {
        let source = "def foo():\n    \"\"\"foo doc\"\"\"\n    return 1\n";
        let tree = parse_code(source);
        let func_node = find_node_by_kind(tree.root_node(), "function_definition").unwrap();

        let symbol = extract_function(func_node, Scope::Module, source, "test.py").unwrap();

        assert_eq!(symbol.name, "foo");
        assert_eq!(symbol.kind, SymbolKind::Function);
        assert_eq!(symbol.doc, "\"\"\"foo doc\"\"\"");
        assert_eq!(symbol.start_line, 1);
        assert_eq!(symbol.end_line, 3);
    }

    #[test]
    fn test_success_method() {
        let source = "class Foo:\n    def bar(self):\n        \"\"\"bar doc\"\"\"\n        pass\n";
        let tree = parse_code(source);
        let method_node = find_node_by_kind(tree.root_node(), "function_definition").unwrap();

        let symbol = extract_function(method_node, Scope::Class, source, "test.py").unwrap();

        assert_eq!(symbol.name, "bar");
        assert_eq!(symbol.kind, SymbolKind::Method);
        assert_eq!(symbol.doc, "\"\"\"bar doc\"\"\"");
        assert_eq!(symbol.start_line, 2);
        assert_eq!(symbol.end_line, 4);
    }
}

#[cfg(test)]
mod extract_docstring_tests {
    use super::*;

    fn find_node_by_kind<'a>(node: tree_sitter::Node<'a>, kind: &str) -> Option<tree_sitter::Node<'a>> {
        if node.kind() == kind {
            return Some(node);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if let Some(found) = find_node_by_kind(child, kind) {
                return Some(found);
            }
        }
        None
    }

    #[test]
    fn test_body_without_docstring() {
        let source = "class Foo:\n    x = 1\n";
        let tree = parse_code(source);
        let class_node = find_node_by_kind(tree.root_node(), "class_definition").unwrap();

        assert_eq!(extract_docstring(class_node, source), "");
    }

    #[test]
    fn test_with_docstring() {
        let source = "def foo():\n    \"\"\"my doc\"\"\"\n    pass\n";
        let tree = parse_code(source);
        let func_node = find_node_by_kind(tree.root_node(), "function_definition").unwrap();

        assert_eq!(extract_docstring(func_node, source), "\"\"\"my doc\"\"\"");
    }

    #[test]
    fn test_without_body() {
        let source = "x = 1";
        let tree = parse_code(source);

        assert_eq!(extract_docstring(tree.root_node(), source), "");
    }
}

#[cfg(test)]
mod extract_name_tests {
    use super::*;

    fn find_node_by_kind<'a>(node: tree_sitter::Node<'a>, kind: &str) -> Option<tree_sitter::Node<'a>> {
        if node.kind() == kind {
            return Some(node);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if let Some(found) = find_node_by_kind(child, kind) {
                return Some(found);
            }
        }
        None
    }

    #[test]
    fn test_missing_name_field() {
        let source = "x = 1";
        let tree = parse_code(source);

        let result = extract_name_from_symbol(tree.root_node(), source, "test.py");
        assert_eq!(
            result.unwrap_err(),
            ExtractionIssue::SourceCodeError(SourceCodeIssue::InvalidSyntax {
                path: "test.py".to_string(),
                reason: "Missing name in line '1'".to_string(),
            })
        );
    }

    #[test]
    fn test_success() {
        let source = "def foo():\n    pass\n";
        let tree = parse_code(source);
        let func_node = find_node_by_kind(tree.root_node(), "function_definition").unwrap();

        assert_eq!(extract_name_from_symbol(func_node, source, "test.py").unwrap(), "foo");
    }
}

