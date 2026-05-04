/// Contract test suite for the `Adapter` trait.
///
/// Each adapter must pass these tests to ensure consistent behavior.
/// Use the `adapter_contract_tests!` macro in your adapter's test module.
///
/// Parameters:
/// - `adapter`: expression that creates the adapter instance
/// - `extension`: file extension handled by this adapter (e.g., ".py")
/// - `valid_source`: source code that produces at least one symbol when extracted
/// - `ignored_filename`: a filename that should be ignored by this adapter

macro_rules! adapter_contract_tests {
    (
        adapter: $adapter:expr,
        extension: $ext:expr,
        valid_source: $source:expr,
        ignored_filename: $ignored:expr $(,)?
    ) => {
        mod contract {
            use super::*;
            use crate::analysis::connector::Adapter;
            use crate::model::{AnalysisWarning, ExtractionIssue, RetryableIssue};
            use std::io::Write;
            use tempfile::tempdir;

            fn make_adapter() -> impl Adapter {
                $adapter
            }

            fn extract_from_source() -> crate::model::RawModule {
                let dir = tempdir().unwrap();
                let file_path = dir.path().join(format!("test{}", $ext));
                let mut file = std::fs::File::create(&file_path).unwrap();
                write!(file, "{}", $source).unwrap();
                drop(file);

                let url = file_path.to_string_lossy().to_string();
                make_adapter()
                    .extract(&url)
                    .expect("contract: extract on valid source should succeed")
            }


            #[test]
            fn supported_extensions_is_not_empty() {
                assert!(
                    !make_adapter().supported_extensions().is_empty(),
                    "adapter must support at least one extension",
                );
            }

            #[test]
            fn can_handle_supported_extension() {
                let path = format!("some/path/file{}", $ext);
                assert!(
                    make_adapter().can_handle(&path),
                    "adapter should handle files with extension '{}'", $ext,
                );
            }

            #[test]
            fn cannot_handle_unknown_extension() {
                assert!(
                    !make_adapter().can_handle("file.unknown_ext_xyz"),
                    "adapter should not handle unknown extensions",
                );
            }


            #[test]
            fn should_ignore_ignored_file() {
                let path = format!("some/path/{}", $ignored);
                assert!(
                    make_adapter().should_ignore(&path),
                    "adapter should ignore '{}'", $ignored,
                );
            }

            #[test]
            fn should_not_ignore_regular_file() {
                let path = format!("some/path/regular{}", $ext);
                assert!(
                    !make_adapter().should_ignore(&path),
                    "adapter should not ignore regular files",
                );
            }

            #[test]
            fn extract_ignored_file_returns_warning() {
                let path = format!("some/path/{}", $ignored);
                let result = make_adapter().extract(&path);
                assert!(
                    matches!(
                        result,
                        Err(ExtractionIssue::Warning(AnalysisWarning::IgnoredFile { .. }))
                    ),
                    "extract on ignored file should return Warning::IgnoredFile",
                );
            }

            #[test]
            fn extract_unreadable_file_returns_retryable() {
                let path = format!("/nonexistent/path/file{}", $ext);
                let result = make_adapter().extract(&path);
                assert!(
                    matches!(
                        result,
                        Err(ExtractionIssue::Retryable(RetryableIssue::UnreadableFile { .. }))
                    ),
                    "extract on unreadable file should return Retryable::UnreadableFile",
                );
            }

            #[test]
            fn extract_valid_file_has_correct_path() {
                let dir = tempdir().unwrap();
                let file_path = dir.path().join(format!("test{}", $ext));
                let mut file = std::fs::File::create(&file_path).unwrap();
                write!(file, "{}", $source).unwrap();
                drop(file);

                let url = file_path.to_string_lossy().to_string();
                let module = make_adapter().extract(&url).unwrap();
                assert_eq!(module.relative_path, url);
            }

            #[test]
            fn extract_valid_file_has_non_empty_name() {
                let module = extract_from_source();
                assert!(!module.name.is_empty(), "module name should not be empty");
            }

            #[test]
            fn extract_valid_file_produces_symbols() {
                let module = extract_from_source();
                assert!(
                    !module.symbols.is_empty(),
                    "valid source should produce at least one symbol",
                );
            }

            #[test]
            fn symbols_have_non_empty_names() {
                let module = extract_from_source();
                fn check(symbols: &[crate::model::RawSymbol]) {
                    for sym in symbols {
                        assert!(!sym.name.is_empty(), "symbol name must not be empty");
                        check(&sym.children_symbols);
                    }
                }
                check(&module.symbols);
            }

            #[test]
            fn symbols_have_valid_line_ranges() {
                let module = extract_from_source();
                fn check(symbols: &[crate::model::RawSymbol]) {
                    for sym in symbols {
                        assert!(
                            sym.start_line >= 1,
                            "start_line should be >= 1, got {} for '{}'",
                            sym.start_line, sym.name,
                        );
                        assert!(
                            sym.start_line <= sym.end_line,
                            "start_line ({}) should be <= end_line ({}) for '{}'",
                            sym.start_line, sym.end_line, sym.name,
                        );
                        check(&sym.children_symbols);
                    }
                }
                check(&module.symbols);
            }

            #[test]
            fn symbols_have_non_empty_location() {
                let module = extract_from_source();
                fn check(symbols: &[crate::model::RawSymbol]) {
                    for sym in symbols {
                        assert!(
                            !sym.location.is_empty(),
                            "symbol location must not be empty for '{}'", sym.name,
                        );
                        check(&sym.children_symbols);
                    }
                }
                check(&module.symbols);
            }

            #[test]
            fn relations_have_valid_fields() {
                let module = extract_from_source();
                for rel in &module.relations {
                    assert!(
                        !rel.imported_name.is_empty(),
                        "relation imported_name must not be empty",
                    );
                    assert!(
                        rel.line >= 1,
                        "relation line should be >= 1, got {}", rel.line,
                    );
                    assert!(
                        !rel.source_path.is_empty(),
                        "relation source_path must not be empty",
                    );
                }
            }
        }
    };
}