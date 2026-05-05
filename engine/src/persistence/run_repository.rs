use crate::model::{Run, RunId, RunStatus, ProjectId, ServiceError};

pub trait RunRepository {
    async fn update_status(&self, run: &Run) -> Result<(), ServiceError>;
    async fn claim_next_pending(&self) -> Result<Option<Run>, ServiceError>;
}

pub struct SqlxRunRepository{
    pool: sqlx::PgPool,
}

impl SqlxRunRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl RunRepository for SqlxRunRepository {
    async fn update_status(&self, run: &Run) -> Result<(), ServiceError> {
        sqlx::query("UPDATE analysis_run SET status=$1, started_at=$2, finished_at=$3, error_message=$4 WHERE id = $5")
            .bind(run.status.to_string())
            .bind(run.started_at.as_ref())
            .bind(run.finished_at.as_ref())
            .bind(run.error_message.as_ref())
            .bind(run.id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;
        Ok(())
    }

    async fn claim_next_pending(&self) -> Result<Option<Run>, ServiceError> {
        let row = sqlx::query(
            "UPDATE analysis_run
             SET status = 'processing', started_at = NOW()
             WHERE id = (
                 SELECT id FROM analysis_run
                 WHERE status = 'pending'
                 ORDER BY created_at
                 LIMIT 1
                 FOR UPDATE SKIP LOCKED
             )
             RETURNING *"
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;

        match row {
            None => Ok(None),
            Some(row) => {
                use sqlx::Row;
                let status: String = row.get("status");
                let run = Run {
                    id: RunId::from_raw(row.get("id")),
                    project_id: ProjectId::from_raw(row.get("project_id")),
                    branch: row.get("branch"),
                    commit: row.get("commit"),
                    status: status.parse::<RunStatus>()
                        .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?,
                    error_message: row.get("error_message"),
                    started_at: row.get("started_at"),
                    finished_at: row.get("finished_at"),
                };
                Ok(Some(run))
            }
        }
    }
}