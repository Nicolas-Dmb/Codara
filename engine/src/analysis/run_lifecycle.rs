use crate::model::{Run, ServiceError};
use crate::persistence::RunRepository;

pub struct RunLifecycle<R: RunRepository> {
    run_repo: R,
    run: Run
}

impl<R: RunRepository> RunLifecycle<R> {
    pub fn new(run_repo: R, run: Run) -> Self {
        Self { run_repo, run }
    }

    pub async fn mark_as_failed(&mut self, error_message: String) -> Result<(), ServiceError> {
        self.run.fail(error_message);
        self.run_repo.update_status(&self.run).await
    }

    pub async fn mark_as_done(&mut self) -> Result<(), ServiceError> {
        self.run.succeed();
        self.run_repo.update_status(&self.run).await
    }

    pub async fn mark_as_partial_success(&mut self) -> Result<(), ServiceError> {
        self.run.partial_success();
        self.run_repo.update_status(&self.run).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use crate::model::{ProjectId, RunStatus};

    struct FakeRunRepository {
        last_status: Mutex<Option<String>>,
        should_fail: bool,
    }

    impl FakeRunRepository {
        fn new() -> Self {
            Self { last_status: Mutex::new(None), should_fail: false }
        }

        fn failing() -> Self {
            Self { last_status: Mutex::new(None), should_fail: true }
        }

        fn last_status(&self) -> Option<String> {
            self.last_status.lock().unwrap().clone()
        }
    }

    impl RunRepository for FakeRunRepository {
        async fn update_status(&self, run: &Run) -> Result<(), ServiceError> {
            if self.should_fail {
                return Err(ServiceError::DatabaseRequestFailed("db error".to_string()));
            }
            *self.last_status.lock().unwrap() = Some(run.status.to_string());
            Ok(())
        }

        async fn claim_next_pending(&self) -> Result<Option<Run>, ServiceError> {
            Ok(None)
        }
    }

    fn make_run() -> Run {
        let project_id = ProjectId::new("ns".to_string(), "proj".to_string());
        Run::new(project_id, "main".to_string(), "abc123".to_string())
    }

    fn make_service() -> RunLifecycle<FakeRunRepository> {
        RunLifecycle::new(FakeRunRepository::new(), make_run())
    }

    fn make_failing_service() -> RunLifecycle<FakeRunRepository> {
        RunLifecycle::new(FakeRunRepository::failing(), make_run())
    }

    #[tokio::test]
    async fn mark_as_done_sets_status() {
        let mut svc = make_service();
        let result = svc.mark_as_done().await;
        assert!(result.is_ok());
        assert_eq!(svc.run.status, RunStatus::Done);
        assert!(svc.run.finished_at.is_some());
        assert_eq!(svc.run_repo.last_status(), Some("done".to_string()));
    }

    #[tokio::test]
    async fn mark_as_failed_sets_status_and_error() {
        let mut svc = make_service();
        let result = svc.mark_as_failed("something broke".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(svc.run.status, RunStatus::Failed);
        assert_eq!(svc.run.error_message, Some("something broke".to_string()));
        assert!(svc.run.finished_at.is_some());
        assert_eq!(svc.run_repo.last_status(), Some("failed".to_string()));
    }

    #[tokio::test]
    async fn mark_as_partial_success_sets_status() {
        let mut svc = make_service();
        let result = svc.mark_as_partial_success().await;
        assert!(result.is_ok());
        assert_eq!(svc.run.status, RunStatus::PartialSuccess);
        assert!(svc.run.finished_at.is_some());
        assert_eq!(svc.run_repo.last_status(), Some("partial_success".to_string()));
    }

    #[tokio::test]
    async fn repo_error_is_propagated() {
        let mut svc = make_failing_service();
        let result = svc.mark_as_done().await;
        assert!(result.is_err());
    }
}