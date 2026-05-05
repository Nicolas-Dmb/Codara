use std::path::Path;
use std::process::Command;
use crate::model::{Project, RunError};

pub trait Cloner {
    fn clone_repo(&self, project: &Project, target: &Path) -> Result<(), RunError>;
}

pub struct ShellCloner;

impl ShellCloner {
    pub fn new() -> Self {
        Self
    }
}

impl Cloner for ShellCloner {
    fn clone_repo(&self, project: &Project, target: &Path) -> Result<(), RunError> {
        let output = Command::new("bash")
            .arg("clone.sh")
            .arg(&project.repo_url)
            .arg(&project.branch)
            .arg(target)
            .output()
            .map_err(|e| RunError::CloneFailed(e.to_string()))?;
        if !output.status.success() {
            return Err(RunError::CloneFailed(String::from_utf8_lossy(&output.stderr).to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ProjectId;
    use tempfile::TempDir;

    fn make_project(repo_url: &str, branch: &str) -> Project {
        let id = ProjectId::new("ns".to_string(), "proj".to_string());
        Project::from_db(id, "proj".to_string(), repo_url.to_string(), branch.to_string())
    }

    #[test]
    fn clone_fails_with_invalid_path() {
        let cloner = ShellCloner::new();
        let project = make_project("/nonexistent/repo/path", "main");
        let tmp_dir = TempDir::new().unwrap();

        let result = cloner.clone_repo(&project, tmp_dir.path());

        assert!(matches!(result, Err(RunError::CloneFailed(_))));
    }

    #[test]
    fn clone_succeeds_with_local_repo() {
        let source_dir = TempDir::new().unwrap();
        let source_path = source_dir.path();

        Command::new("git").args(["init"]).current_dir(source_path).output().unwrap();
        Command::new("git").args(["config", "user.email", "test@test.com"]).current_dir(source_path).output().unwrap();
        Command::new("git").args(["config", "user.name", "Test"]).current_dir(source_path).output().unwrap();
        std::fs::write(source_path.join("file.txt"), "hello").unwrap();
        Command::new("git").args(["add", "."]).current_dir(source_path).output().unwrap();
        Command::new("git").args(["commit", "-m", "init"]).current_dir(source_path).output().unwrap();
        Command::new("git").args(["branch", "-M", "main"]).current_dir(source_path).output().unwrap();

        let cloner = ShellCloner::new();
        let project = make_project(&source_path.to_string_lossy(), "main");
        let clone_dir = TempDir::new().unwrap();

        let result = cloner.clone_repo(&project, clone_dir.path());

        assert!(result.is_ok());
        assert!(clone_dir.path().join("file.txt").exists());
    }
}