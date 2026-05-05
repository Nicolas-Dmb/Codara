use std::time::Duration;
use sqlx::PgPool;
use crate::persistence::{SqlxAnalysisRepository, SqlxRunRepository, SqlxProjectRepository, RunRepository, ProjectRepository};
use crate::services::Context;
use crate::analysis::runner::run_analysis;

const POLL_INTERVAL: Duration = Duration::from_mins(1);

pub async fn start_listener(pool: PgPool) {

    loop {
        let run_repo = SqlxRunRepository::new(pool.clone());

        match run_repo.claim_next_pending().await {
            Ok(Some(run)) => {
                println!("Claimed run {}", run.id);

                let project_repo = SqlxProjectRepository::new(pool.clone());
                match project_repo.find_by_id(&run.project_id).await {
                    Ok(project) => {
                        let analysis_repo = SqlxAnalysisRepository::new(pool.clone());
                        let context = Context::new(analysis_repo, run_repo);
                        run_analysis(context, run, project).await;
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch project for run {}: {}", run.id, e);
                    }
                }
            }
            Ok(None) => {}
            Err(e) => {
                eprintln!("Error claiming next run: {}", e);
            }
        }

        tokio::time::sleep(POLL_INTERVAL).await;
    }
}