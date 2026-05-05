use crate::model::{Project, ProjectId, ServiceError};

pub trait ProjectRepository {
    async fn find_by_id(&self, id: &ProjectId) -> Result<Project, ServiceError>;
}

pub struct SqlxProjectRepository {
    pool: sqlx::PgPool,
}

impl SqlxProjectRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl ProjectRepository for SqlxProjectRepository {
    async fn find_by_id(&self, id: &ProjectId) -> Result<Project, ServiceError> {
        let row = sqlx::query("SELECT id, name, repo_url, branch FROM project WHERE id = $1")
            .bind(id.to_string())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;

        use sqlx::Row;
        Ok(Project::from_db(
            ProjectId::from_raw(row.get("id")),
            row.get("name"),
            row.get("repo_url"),
            row.get("branch"),
        ))
    }
}