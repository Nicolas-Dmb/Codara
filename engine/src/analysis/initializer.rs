use crate::model;
use model::{Run, Project, RunError};
use crate::persistence::{AnalysisRepository, RunRepository};
use crate::services::Context;
use crate::analysis;
use std::process::Command;
use tempfile::TempDir;

pub async fn initializer<A: AnalysisRepository, R: RunRepository>(context: Context<A, R>, run: Run, project: Project) {
    let Context { analysis_repo, run_repo } = context;
    let run_id = run.id.clone();
    let mut lifecycle = analysis::run_lifecycle::RunLifecycle::new(run_repo, run);

    lifecycle.mark_as_processing().await.expect("Failed to mark run as processing");

    // Step 1: Clone the repository into a temporary directory
    let tmp_dir = match TempDir::new() {
        Ok(dir) => dir,
        Err(e) => {
            lifecycle.mark_as_failed(format!("Failed to create temp directory: {}", e)).await.expect("Failed to mark run as failed");
            return;
        }
    };

    let clone_result = clone_repository(&project, &tmp_dir);
    if let Err(e) = clone_result {
        lifecycle.mark_as_failed(format!("Failed to clone repository: {}", e)).await.expect("Failed to mark run as failed");
        return;
    }

    // Step 2: Walk the repository and extract modules
    let adapters_registry = analysis::connector::AdapterRegistry::new();
    let walk_result = analysis::walker::walk(
        tmp_dir.path(),
        true,
        &adapters_registry,
    );
    if let Err(e) = walk_result {
        lifecycle.mark_as_failed(format!("Failed to walk repository: {}", e)).await.expect("Failed to mark run as failed");
        return;
    }
    let is_partial_success = walk_result.as_ref().unwrap().has_retryables();

    // Step 3: Store the extracted modules in the database
    let adapter = analysis::raw_adapter::RawAdapter::new(analysis_repo, project.id.clone(), run_id.clone());
    let convert_result = adapter.convert_and_store(walk_result.unwrap()).await;
    if let Err(e) = convert_result {
        lifecycle.mark_as_failed(format!("Failed to convert and store analysis results: {}", e)).await.expect("Failed to mark run as failed");
        return;
    }

    // Step 4: Mark the run as completed
    if is_partial_success {
        lifecycle.mark_as_partial_success().await.expect("Failed to mark run as partially successful");
    } else {
        lifecycle.mark_as_done().await.expect("Failed to mark run as successful");
    }

}

fn clone_repository(project: &Project, tmp_dir: &TempDir) -> Result<(), RunError> {
    let output = Command::new("bash")
        .arg("clone.sh")
        .arg(&project.repo_url)
        .arg(&project.branch)
        .arg(tmp_dir.path())
        .output()
        .map_err(|e| RunError::CloneFailed(e.to_string()))?;
    if !output.status.success() {
        return Err(RunError::CloneFailed(String::from_utf8_lossy(&output.stderr).to_string()));
    }
    Ok(())
}