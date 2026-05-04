use crate::model::{Run, ServiceError};

pub trait RunRepository {
    async fn update_status(&self, run: &Run) -> Result<(), ServiceError>;
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
}