use crate::model;
use model::{Run, Project, RunError};
use crate::persistence::{AnalysisRepository, RunRepository, ProjectRepository};
use crate::services::{Context, Cloner};
use crate::analysis;
use tempfile::TempDir;
use tracing::{instrument, info};


#[instrument(name = "run_analysis", skip(context))]
pub async fn run_analysis<A: AnalysisRepository, R: RunRepository, P: ProjectRepository, C: Cloner>(context: Context<A, R, P, C>, run: Run) {
    let Context { analysis_repo, run_repo, project_repo, cloner } = context;
    let run_id = run.id.clone();
    let project_id = run.project_id.clone();
    let mut lifecycle = analysis::run_lifecycle::RunLifecycle::new(run_repo, run);

    // Step 0: request the project details from the database
    info!("Fetching project details for project {}", project_id);
    let project = match project_repo.find_by_id(&project_id).await{
        Ok(project) => project,
        Err(e) => {
            lifecycle.mark_as_failed(format!("Failed to fetch project details: {}", e)).await.expect("Failed to mark run as failed");
            return;
        }
    };

    // Step 1: Clone the repository into a temporary directory
    info!("Cloning repository {} (branch {})", project.id, project.branch);
    let tmp_dir = match TempDir::new() {
        Ok(dir) => dir,
        Err(e) => {
            lifecycle.mark_as_failed(format!("Failed to create temp directory: {}", e)).await.expect("Failed to mark run as failed");
            return;
        }
    };

    if let Err(e) = cloner.clone_repo(&project, tmp_dir.path()) {
        lifecycle.mark_as_failed(format!("Failed to clone repository: {}", e)).await.expect("Failed to mark run as failed");
        return;
    }

    // Step 2: Walk the repository and extract modules
    info!("Walking repository and extracting modules");
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
    info!("Storing extracted modules in the database");
    let adapter = analysis::raw_adapter::RawAdapter::new(analysis_repo, project.id.clone(), run_id.clone());
    let convert_result = adapter.convert_and_store(walk_result.unwrap()).await;
    if let Err(e) = convert_result {
        lifecycle.mark_as_failed(format!("Failed to convert and store analysis results: {}", e)).await.expect("Failed to mark run as failed");
        return;
    }

    // Step 4: Mark the run as completed
    if is_partial_success {
        lifecycle.mark_as_partial_success().await.expect("Failed to mark run as partially successful");
        info!("Run completed with partial success due to retryable issues");
    } else {
        lifecycle.mark_as_done().await.expect("Failed to mark run as successful");
        info!("Run completed successfully");
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::sync::{Arc, Mutex};
    use crate::model::{ProjectId, ServiceError, Module, Symbol, Relation, AnalysisWarning, RetryableIssue, SourceCodeIssue};

    // --- Fake Repositories ---

    struct FakeRunRepository {
        last_status: Arc<Mutex<Option<String>>>,
        last_error: Arc<Mutex<Option<String>>>,
    }

    impl FakeRunRepository {
        fn new() -> Self {
            Self {
                last_status: Arc::new(Mutex::new(None)),
                last_error: Arc::new(Mutex::new(None)),
            }
        }
    }

    impl RunRepository for FakeRunRepository {
        async fn update_status(&self, run: &Run) -> Result<(), ServiceError> {
            *self.last_status.lock().unwrap() = Some(run.status.to_string());
            *self.last_error.lock().unwrap() = run.error_message.clone();
            Ok(())
        }

        async fn claim_next_pending(&self) -> Result<Option<Run>, ServiceError> {
            Ok(None)
        }
    }

    struct FakeProjectRepository {
        project: Mutex<Option<Project>>,
    }

    impl FakeProjectRepository {
        fn returning(project: Project) -> Self {
            Self { project: Mutex::new(Some(project)) }
        }

        fn failing() -> Self {
            Self { project: Mutex::new(None) }
        }
    }

    impl ProjectRepository for FakeProjectRepository {
        async fn find_by_id(&self, _id: &ProjectId) -> Result<Project, ServiceError> {
            self.project.lock().unwrap().take()
                .ok_or(ServiceError::DatabaseRequestFailed("project not found".to_string()))
        }
    }

    struct FakeAnalysisRepository {
        stored: Arc<Mutex<bool>>,
    }

    impl FakeAnalysisRepository {
        fn new() -> Self {
            Self { stored: Arc::new(Mutex::new(false)) }
        }
    }

    impl AnalysisRepository for FakeAnalysisRepository {
        async fn store_batch(
            &self,
            _run_id: &str,
            _modules: &[Module],
            _symbols: &[Symbol],
            _relations: &[Relation],
            _warnings: &[AnalysisWarning],
            _retryable_issues: &[RetryableIssue],
            _source_code_issues: &[SourceCodeIssue],
        ) -> Result<(), ServiceError> {
            *self.stored.lock().unwrap() = true;
            Ok(())
        }
    }

    struct FailingAnalysisRepository;

    impl AnalysisRepository for FailingAnalysisRepository {
        async fn store_batch(
            &self,
            _run_id: &str,
            _modules: &[Module],
            _symbols: &[Symbol],
            _relations: &[Relation],
            _warnings: &[AnalysisWarning],
            _retryable_issues: &[RetryableIssue],
            _source_code_issues: &[SourceCodeIssue],
        ) -> Result<(), ServiceError> {
            Err(ServiceError::DatabaseRequestFailed("store failed".to_string()))
        }
    }

    // --- Fake Cloner ---

    struct FakeCloner {
        should_fail: bool,
    }

    impl FakeCloner {
        fn succeeding() -> Self {
            Self { should_fail: false }
        }

        fn failing() -> Self {
            Self { should_fail: true }
        }
    }

    impl Cloner for FakeCloner {
        fn clone_repo(&self, _project: &Project, target: &Path) -> Result<(), RunError> {
            if self.should_fail {
                return Err(RunError::CloneFailed("fake clone error".to_string()));
            }
            // Create a minimal Python file so the walker finds data
            std::fs::write(target.join("main.py"), "def hello():\n    pass\n").unwrap();
            Ok(())
        }
    }

    // --- Helpers ---

    fn test_project_id() -> ProjectId {
        ProjectId::new("ns".to_string(), "proj".to_string())
    }

    fn make_run() -> Run {
        Run::new(test_project_id(), "main".to_string(), "abc123".to_string())
    }

    fn make_project() -> Project {
        Project::from_db(
            test_project_id(),
            "proj".to_string(),
            "https://example.com/ns/proj.git".to_string(),
            "main".to_string(),
        )
    }

    // --- Tests ---

    #[tokio::test]
    async fn marks_failed_when_project_not_found() {
        let run_repo = FakeRunRepository::new();
        let status = run_repo.last_status.clone();
        let error = run_repo.last_error.clone();

        let context = Context::new(FakeAnalysisRepository::new(), run_repo, FakeProjectRepository::failing(), FakeCloner::succeeding());
        run_analysis(context, make_run()).await;

        assert_eq!(status.lock().unwrap().as_deref(), Some("failed"));
        assert!(error.lock().unwrap().as_ref().unwrap().contains("project not found"));
    }

    #[tokio::test]
    async fn marks_failed_when_clone_fails() {
        let run_repo = FakeRunRepository::new();
        let status = run_repo.last_status.clone();
        let error = run_repo.last_error.clone();

        let context = Context::new(FakeAnalysisRepository::new(), run_repo, FakeProjectRepository::returning(make_project()), FakeCloner::failing());
        run_analysis(context, make_run()).await;

        assert_eq!(status.lock().unwrap().as_deref(), Some("failed"));
        assert!(error.lock().unwrap().as_ref().unwrap().contains("clone"));
    }

    #[tokio::test]
    async fn marks_failed_when_store_fails() {
        let run_repo = FakeRunRepository::new();
        let status = run_repo.last_status.clone();
        let error = run_repo.last_error.clone();

        let context = Context::new(FailingAnalysisRepository, run_repo, FakeProjectRepository::returning(make_project()), FakeCloner::succeeding());
        run_analysis(context, make_run()).await;

        assert_eq!(status.lock().unwrap().as_deref(), Some("failed"));
        assert!(error.lock().unwrap().as_ref().unwrap().contains("store"));
    }

    #[tokio::test]
    async fn marks_done_on_success() {
        let run_repo = FakeRunRepository::new();
        let status = run_repo.last_status.clone();
        let analysis_repo = FakeAnalysisRepository::new();
        let stored = analysis_repo.stored.clone();

        let context = Context::new(analysis_repo, run_repo, FakeProjectRepository::returning(make_project()), FakeCloner::succeeding());
        run_analysis(context, make_run()).await;

        assert_eq!(status.lock().unwrap().as_deref(), Some("done"));
        assert!(*stored.lock().unwrap());
    }
}