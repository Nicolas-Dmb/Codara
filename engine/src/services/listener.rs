use std::time::Duration;
use sqlx::PgPool;
use crate::model::{ Run, ServiceError};
use crate::persistence::{SqlxAnalysisRepository, SqlxRunRepository, SqlxProjectRepository, RunRepository};
use crate::services::Context;
use crate::analysis::runner::run_analysis;

const POLL_INTERVAL: Duration = Duration::from_mins(1);

pub async fn start_listener(pool: PgPool) {
    loop {
        let run_repo = SqlxRunRepository::new(pool.clone());
        let project_repo = SqlxProjectRepository::new(pool.clone());

        match poll_once(&run_repo).await {
            Ok(Some(run)) => {
                println!("Claimed run {}", run.id);
                let analysis_repo = SqlxAnalysisRepository::new(pool.clone());
                let context = Context::new(analysis_repo, run_repo, project_repo);
                run_analysis(context, run).await;
            }
            Ok(None) => {}
            Err(e) => {
                eprintln!("Polling error: {}", e);
            }
        }

        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

async fn poll_once<R: RunRepository>(
    run_repo: &R,
) -> Result<Option<Run>, ServiceError> {
    let run = run_repo.claim_next_pending().await?;
    match run {
        None => Ok(None),
        Some(run) => Ok(Some(run))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ProjectId, RunId, RunStatus};

    struct FakeRunRepository {
        pending_run: Option<Run>,
        should_fail: bool,
    }

    impl FakeRunRepository {
        fn empty() -> Self {
            Self { pending_run: None, should_fail: false }
        }

        fn with_pending(run: Run) -> Self {
            Self { pending_run: Some(run), should_fail: false }
        }

        fn failing() -> Self {
            Self { pending_run: None, should_fail: true }
        }
    }

    impl RunRepository for FakeRunRepository {
        async fn update_status(&self, _run: &Run) -> Result<(), ServiceError> {
            Ok(())
        }

        async fn claim_next_pending(&self) -> Result<Option<Run>, ServiceError> {
            if self.should_fail {
                return Err(ServiceError::DatabaseRequestFailed("claim failed".to_string()));
            }
            match &self.pending_run {
                None => Ok(None),
                Some(r) => Ok(Some(Run {
                    id: RunId::from_raw(r.id.to_string()),
                    project_id: ProjectId::from_raw(r.project_id.to_string()),
                    branch: r.branch.clone(),
                    commit: r.commit.clone(),
                    status: RunStatus::Processing,
                    error_message: None,
                    started_at: None,
                    finished_at: None,
                })),
            }
        }
    }

    fn make_run() -> Run {
        Run::new(
            ProjectId::new("ns".to_string(), "proj".to_string()),
            "main".to_string(),
            "abc123".to_string(),
        )
    }

    #[tokio::test]
    async fn poll_once_returns_none_when_no_pending_run() {
        let run_repo = FakeRunRepository::empty();

        let result = poll_once(&run_repo).await;

        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn poll_once_returns_run_and_project_when_pending() {
        let run_repo = FakeRunRepository::with_pending(make_run());

        let result = poll_once(&run_repo).await;

        let (run) = result.unwrap().unwrap();
        assert_eq!(run.status, RunStatus::Processing);
    }

    #[tokio::test]
    async fn poll_once_returns_error_when_claim_fails() {
        let run_repo = FakeRunRepository::failing();

        let result = poll_once(&run_repo).await;

        assert!(result.is_err());
    }
}