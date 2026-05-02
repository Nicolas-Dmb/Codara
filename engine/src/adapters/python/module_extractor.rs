use crate::model::{ ExtractionIssue, ExtractedItems, SourceCodeIssue};
use crate::adapters::python::symbol_extractor::{extract_function, extract_class};
use std::fmt;
use crate::adapters::python::{Scope, extract_import};

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scope::Module => f.write_str("module"),
            Scope::Class => f.write_str("class"),
            Scope::Function => f.write_str("function"),
        }
    }
}

pub fn extractor( tree: &tree_sitter::Tree, source_code: &str, path: &str) 
    -> Result<ExtractedItems, ExtractionIssue> {
    let mut items = ExtractedItems {
        symbols: Vec::new(),
        relations: Vec::new(),
    };
    let root = tree.root_node();

    visit_node(root, Scope::Module, source_code, &mut items, path)?;

    Ok(items)
}

pub fn visit_node(node: tree_sitter::Node,scope:Scope, source_code: &str, items: &mut ExtractedItems, path: &str)-> Result<(), ExtractionIssue> {
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


#[cfg(test)]
mod extractor_tests {
    use crate::{adapters::python::parser::parse_code, model::SymbolKind};

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
    use crate::{adapters::python::parser::parse_code, model::SymbolKind};

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