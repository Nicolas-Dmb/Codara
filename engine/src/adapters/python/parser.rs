use std::path::Path;
use tree_sitter::{Parser, Tree};

use crate::model::{RetryableIssue, ExtractionIssue};

pub fn parse_code(path: &Path) -> Result<Tree, ExtractionIssue> {
    let code = std::fs::read_to_string(path).map_err(|err| {
        ExtractionIssue::Retryable(RetryableIssue::UnreadableFile {
            path: path.to_string_lossy().to_string(),
            reason: err.to_string(),
        })
    })?;

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_python::LANGUAGE.into())
        .expect("Python grammar ABI mismatch");

    Ok(parser
        .parse(&code, None)
        .expect("tree-sitter parse failed with language set"))
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

        let tree = parse_code(&file_path).unwrap();
        let root = tree.root_node();
        assert_eq!(root.kind(), "module");
        assert!(root.child_count() > 0);
    }

    #[test]
    fn test_parse_code_file_not_found() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("does_not_exist.py");

        let result = parse_code(&file_path);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ExtractionIssue::Retryable(RetryableIssue::UnreadableFile { .. })
        ));
    }
}
