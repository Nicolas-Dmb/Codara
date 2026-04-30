use tree_sitter::{Parser, Tree};

pub fn parse_code(source_code: &str) -> Tree {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_python::LANGUAGE.into())
        .expect("Python grammar ABI mismatch");

    parser
        .parse(source_code, None)
        .expect("tree-sitter parse failed with language set")
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_parse_code_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.py");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "class Foo:\n    def bar(self):\n        pass").unwrap();

        let tree = parse_code(&std::fs::read_to_string(&file_path).unwrap());
        let root = tree.root_node();
        assert_eq!(root.kind(), "module");
        assert!(root.child_count() > 0);
    }
}
