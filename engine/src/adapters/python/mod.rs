use crate::analysis::connector::Adapter;
use crate::model::{AnalysisWarning, ExtractionIssue, RawModule, RawSymbol, RelationKind, SymbolKind, RetryableIssue, SourceCodeIssue};
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
        
        let symbols = extractor(&ast_code, &source_code, url)?;

        symbols.into_iter().for_each(|symbol| raw_module.add_symbol(symbol));

        Ok(raw_module)
    }
}

#[derive(Clone, PartialEq)]
enum Scope {
    Module,
    Class,
}

fn extractor( tree: &tree_sitter::Tree, source_code: &str, path: &str) 
    -> Result<Vec<RawSymbol>, ExtractionIssue> {
    let mut symbols = Vec::new();
    let root = tree.root_node();

    visit_node(root, Scope::Module, source_code, &mut symbols, path)?;

    Ok(symbols)
}

fn visit_node(node: tree_sitter::Node,scope:Scope, source_code: &str, symbols: &mut Vec<RawSymbol>, path: &str)-> Result<(), ExtractionIssue> {
    match node.kind() {
        "class_definition" => {
            symbols.push(extract_class(node, source_code, path)?);
        }
        "function_definition" => {
            symbols.push(extract_function(node, scope.clone(), source_code, path)?);
        }
        _ => {
            for child in node.children(&mut node.walk()) {
                visit_node(child, scope.clone(), source_code, symbols, path)?;
            }
        }
    }
    Ok(())
}

fn extract_class(node: tree_sitter::Node, source_code: &str, path: &str)-> Result<RawSymbol, ExtractionIssue>{
    let class_name = extract_name(node, source_code, path)?;
    let class_doc = extract_docstring(node, source_code, path)?;
    
    let mut symbol = RawSymbol::new(
        class_name,
        SymbolKind::Class,
        class_doc,
        node.start_position().row + 1,
        node.end_position().row + 1,
    );
    let mut children = Vec::new();

    let child_node = node.child_by_field_name("body")
        .ok_or_else(|| ExtractionIssue::SourceCodeError(SourceCodeIssue::InvalidSyntax{
            path: path.to_string(),
            reason: format!("Missing body for class in line '{}'", node.start_position().row + 1),
        }))?;
    visit_node(child_node, Scope::Class, source_code,  &mut children, path)?;
    children
    .into_iter()
    .for_each(|child| symbol.add_child(child));

    Ok(symbol)
}

fn extract_function(node: tree_sitter::Node, scope:Scope, source_code: &str, path: &str)->Result<RawSymbol, ExtractionIssue>{
    let function_name = extract_name(node, source_code, path)?;
    let function_doc = extract_docstring(node, source_code, path)?;
    let kind = if scope == Scope::Class {
        SymbolKind::Method
    } else {
        SymbolKind::Function
    };
    let function_symbol = RawSymbol::new(
        function_name,
        kind,
        function_doc,
        node.start_position().row + 1,
        node.end_position().row + 1,
    );
    Ok(function_symbol)
}


fn extract_docstring(node: tree_sitter::Node, source_code: &str, path: &str) -> Result<String, ExtractionIssue> {
    if let Some(body) = node.child_by_field_name("body") {
        let mut cursor = body.walk();

        if let Some(first_child) = body.children(&mut cursor).next() {
            if first_child.kind() == "expression_statement" {
                if let Some(string_node) = first_child.child(0) {
                    if string_node.kind() == "string" {
                        return string_node
                            .utf8_text(source_code.as_bytes())
                            .map_err(|e| ExtractionIssue::Retryable(RetryableIssue::AdapterFailed{
                                path: path.to_string(),
                                reason: format!("Failed to extract docstring: {}", e),
                            }))
                            .map(|s| s.to_string());
                    }
                }
            }
        }
    }

    Ok("".to_string())
}

fn extract_name(node: tree_sitter::Node, source_code: &str, path: &str) -> Result<String, ExtractionIssue> {
    let name_node = node.child_by_field_name("name").ok_or_else(|| 
        ExtractionIssue::SourceCodeError(SourceCodeIssue::InvalidSyntax{
            path: path.to_string(),
            reason: format!("Missing name in line '{}'", node.start_position().row + 1),
        })
    )?;

    let name = name_node.utf8_text(source_code.as_bytes());
    match name {
        Ok(name) => Ok(name.to_string()),
        Err(e) => {
            Err(ExtractionIssue::Retryable(RetryableIssue::AdapterFailed{
                path:path.to_string(),
                reason: format!("Failed to extract name: {}", e),
            }))
        }
    }
}

