use crate::model::{ExtractionIssue, RawSymbol, SymbolKind, SourceCodeIssue, ExtractedItems};
use crate::adapters::python::{Scope, parse_code};
use crate::adapters::python::module_extractor::visit_node;

pub fn extract_class(node: tree_sitter::Node, source_code: &str, path: &str)-> Result<RawSymbol, ExtractionIssue>{
    let class_name = extract_name_from_symbol(node, source_code, path)?;
    let class_doc = extract_docstring(node, source_code);
    
    let mut symbol = RawSymbol::new(
        class_name,
        SymbolKind::Class,
        class_doc,
        path.to_string(),
        node.start_position().row + 1,
        node.end_position().row + 1,
    );

    extract_symbol_body(node, source_code, path, &mut symbol, Scope::Class)?;

    Ok(symbol)
}

pub fn extract_function(node: tree_sitter::Node, scope:Scope, source_code: &str, path: &str)->Result<RawSymbol, ExtractionIssue>{
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
        path.to_string(),
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
pub fn extract_name_from_symbol(node: tree_sitter::Node, source_code: &str, path: &str) -> Result<String, ExtractionIssue> {
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

