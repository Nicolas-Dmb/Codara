use std::path::Path;
use std::fs;
use crate::analysis::connector;
use crate::model;
use model::project::Project;
use model::error::{RunError, AnalysisReport, RetryableIssue};

fn read_directory(
    path: &Path,
    is_root: bool,
) -> Result<Result<std::fs::ReadDir, AnalysisReport>, RunError> {
    match fs::read_dir(path) {
        Ok(entries) => Ok(Ok(entries)),

        Err(e) => {
            if is_root {
                return Err(RunError::ReadRepositoryFailed(e.to_string()));
            }

            let mut report = AnalysisReport::new();

            report.add_retryable(RetryableIssue::UnreadableDirectory {
                path: path.to_string_lossy().to_string(),
                reason: e.to_string(),
            });

            Ok(Err(report))
        }
    }
}

 
pub fn walk(
    run: &model::run::Run,
    project: &Project,
    path: &Path,
    is_root: bool,
) -> Result<AnalysisReport, RunError> {
    let mut report = AnalysisReport::new();

    let entries = match read_directory(path, is_root)? {
        Ok(entries) => entries,
        Err(local_report) => return Ok(local_report),
    };

    let mut has_entry = false;

    let adapters_registry = connector::AdapterRegistry::new();

    for entry in entries {
        has_entry = true;

        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                report.add_retryable(RetryableIssue::UnreadableDirectory {
                    path: path.to_string_lossy().to_string(),
                    reason: e.to_string(),
                });

                continue;
            }
        };

        let entry_path = entry.path();

        if entry_path.is_file() {
            adapters_registry.find_and_extract(&entry_path.to_string_lossy().to_string(), &mut report);
            continue;
        }

        if entry_path.is_dir() {
            let child_report = walk(run, project, &entry_path, false)?;
            report.merge(child_report);
        }
    }

    if is_root && !has_entry {
        return Err(RunError::EmptyRepository);
    }

    Ok(report)
}

#[cfg(test)]
mod walker_tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

    use crate::model::error::{AnalysisReport, RetryableIssue, RunError};
    use crate::model::project::Project;
    use crate::model::run::Run;

    fn setup() -> (Project, Run) {
        let project = Project::new("https://github.com/test/repo.git".to_string()).unwrap();
        let run = Run::new(project.id.clone(), "main".to_string(), "commit123".to_string());

        (project, run)
    }

    #[test]
    fn test_with_empty_directory() {
        let temp_dir = tempdir().unwrap();
        let (project, run) = setup();

        assert_eq!(
            walk(&run, &project, temp_dir.path(), true),
            Err(RunError::EmptyRepository)
        );
    }

    #[test]
    fn test_with_unreadable_directory() {
        let temp_dir = tempdir().unwrap();

        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        fs::set_permissions(&subdir, fs::Permissions::from_mode(0o000)).unwrap();

        let (project, run) = setup();
        let result = walk(&run, &project, temp_dir.path(), true).unwrap();

        assert_eq!(result.retryables.len(), 1);
        assert_eq!(result.warnings.len(), 0);

        match &result.retryables[0] {
            RetryableIssue::UnreadableDirectory { path, reason: _ } => {
                assert_eq!(path, &subdir.to_string_lossy().to_string());
            }
            _ => panic!("Expected unreadable directory retryable issue"),
        }

        fs::set_permissions(&subdir, fs::Permissions::from_mode(0o755)).unwrap();
    }

    #[test]
    fn test_with_unreadable_root_directory() {
        let temp_dir = tempdir().unwrap();
        fs::set_permissions(temp_dir.path(), fs::Permissions::from_mode(0o000)).unwrap();

        let (project, run) = setup();

        assert!(matches!(
            walk(&run, &project, temp_dir.path(), true),
            Err(RunError::ReadRepositoryFailed(_))
        ));

        fs::set_permissions(temp_dir.path(), fs::Permissions::from_mode(0o755)).unwrap();
    }

    #[test]
    fn test_with_valid_files() {
        let temp_dir = tempdir().unwrap();

        fs::write(temp_dir.path().join("file1.py"), "content1").unwrap();

        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        let file2 = subdir.join("file2.py");
        fs::write(&file2, "content2").unwrap();

        let (project, run) = setup();

        assert_eq!(
            walk(&run, &project, temp_dir.path(), true),
            Ok(AnalysisReport::new())
        );
    }
}
