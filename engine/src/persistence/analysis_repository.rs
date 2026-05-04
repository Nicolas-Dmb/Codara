use sqlx::Postgres;
use crate::model::{AnalysisWarning, Module, Relation, RetryableIssue, ServiceError, SourceCodeIssue, Symbol, run};

pub trait AnalysisRepository {
    async fn store_batch(
        &self,
        run_id: &str,
        modules: &[Module],
        symbols: &[Symbol],
        relations: &[Relation],
        warnings: &[AnalysisWarning],
        retryable_issues: &[RetryableIssue],
        source_code_issues: &[SourceCodeIssue],
    ) -> Result<(), ServiceError>;
}

pub struct SqlxAnalysisRepository {
    pool: sqlx::PgPool,
}

impl SqlxAnalysisRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    async fn store_modules(
        tx: &mut sqlx::Transaction<'_, Postgres>,
        modules: &[Module],
    ) -> Result<(), ServiceError> {
        for module in modules {
            sqlx::query(
                "INSERT INTO modules (id, run_id, relative_path, name) VALUES ($1, $2, $3, $4)"
            )
            .bind(module.id.to_string())
            .bind(module.run_id.to_string())
            .bind(&module.relative_path)
            .bind(&module.name)
            .execute(&mut **tx)
            .await
            .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;
        }
        Ok(())
    }

    async fn store_symbols(
        tx: &mut sqlx::Transaction<'_, Postgres>,
        symbols: &[Symbol],
    ) -> Result<(), ServiceError> {
        for symbol in symbols {
            sqlx::query(
                "INSERT INTO symbols (id, module_id, run_id, parent_symbol_id, name, kind, doc, location, start_line, end_line) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
            )
            .bind(symbol.id.to_string())
            .bind(symbol.module_id.to_string())
            .bind(symbol.run_id.to_string())
            .bind(symbol.parent_symbol_id.as_ref().map(|id| id.to_string()))
            .bind(&symbol.name)
            .bind(symbol.kind.to_string())
            .bind(&symbol.doc)
            .bind(&symbol.location)
            .bind(symbol.start_line as i64)
            .bind(symbol.end_line as i64)
            .execute(&mut **tx)
            .await
            .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;
        }
        Ok(())
    }

    async fn store_relations(
        tx: &mut sqlx::Transaction<'_, Postgres>,
        relations: &[Relation],
    ) -> Result<(), ServiceError> {
        for relation in relations {
            sqlx::query(
                "INSERT INTO relations (id, module_id, run_id, parent_symbol_id, imported_name, source_path, target_symbol_id, kind, line) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
            )
            .bind(relation.id.to_string())
            .bind(relation.module_id.to_string())
            .bind(relation.run_id.to_string())
            .bind(relation.parent_symbol_id.as_ref().map(|id| id.to_string()))
            .bind(&relation.imported_name)
            .bind(&relation.source_path)
            .bind(relation.target_symbol_id.as_ref().map(|id| id.to_string()))
            .bind(relation.kind.to_string())
            .bind(relation.line as i64)
            .execute(&mut **tx)
            .await
            .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;
        }
        Ok(())
    }

    async fn store_warnings(
        tx: &mut sqlx::Transaction<'_, Postgres>,
        run_id: &str,
        warnings: &[AnalysisWarning],
    ) -> Result<(), ServiceError> {
        for warning in warnings {
            let (error_kind, path) = match warning {
                AnalysisWarning::UnsupportedFileType { path } => ("unsupported_file_type", path.as_str()),
                AnalysisWarning::IgnoredFile { path } => ("ignored_file", path.as_str()),
            };

            sqlx::query(
                "INSERT INTO analysis_warning (run_id, kind, path) VALUES ($1, $2, $3)"
            )
            .bind(run_id)
            .bind(error_kind)
            .bind(path)
            .execute(&mut **tx)
            .await
            .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;
        }
        Ok(())
    }

    async fn store_retryable_issues(
        tx: &mut sqlx::Transaction<'_, Postgres>,
        run_id: &str,
        issues: &[RetryableIssue],
    ) -> Result<(), ServiceError> {
        for issue in issues {
            let (error_kind, path, reason) = match issue {
                RetryableIssue::UnreadableDirectory { path, reason } => ("unreadable_directory", path.as_str(), reason.as_str()),
                RetryableIssue::UnreadableFile { path, reason } => ("unreadable_file", path.as_str(), reason.as_str()),
                RetryableIssue::AdapterFailed { path, reason } => ("adapter_failed", path.as_str(), reason.as_str()),
                RetryableIssue::UnresolvedImport { path, import_name } => ("unresolved_import", path.as_str(), import_name.as_str()),
            };

            sqlx::query(
                "INSERT INTO retryable_issue (run_id, kind, path, reason) VALUES ($1, $2, $3, $4)"
            )
            .bind(run_id)
            .bind(error_kind)
            .bind(path)
            .bind(reason)
            .execute(&mut **tx)
            .await
            .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;
        }
        Ok(())
    }

    async fn store_source_code_issues(
        tx: &mut sqlx::Transaction<'_, Postgres>,
        run_id: &str,
        issues: &[SourceCodeIssue],
    ) -> Result<(), ServiceError> {
        for issue in issues {
            let (error_kind, path, reason) = match issue {
                SourceCodeIssue::InvalidSyntax { path, reason } => ("invalid_syntax", path.as_str(), reason.as_str()),
            };

            sqlx::query(
                "INSERT INTO source_code_issue (run_id, kind, path, reason) VALUES ($1, $2, $3, $4)"
            )
            .bind(run_id)
            .bind(error_kind)
            .bind(path)
            .bind(reason)
            .execute(&mut **tx)
            .await
            .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;
        }
        Ok(())
    }
}

impl AnalysisRepository for SqlxAnalysisRepository {
    async fn store_batch(
        &self,
        run_id: &str,
        modules: &[Module],
        symbols: &[Symbol],
        relations: &[Relation],
        warnings: &[AnalysisWarning],
        retryable_issues: &[RetryableIssue],
        source_code_issues: &[SourceCodeIssue],
    ) -> Result<(), ServiceError> {
        let mut tx = self.pool.begin()
            .await
            .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;

        Self::store_modules(&mut tx, modules).await?;
        Self::store_symbols(&mut tx, symbols).await?;
        Self::store_relations(&mut tx, relations).await?;


        Self::store_warnings(&mut tx, &run_id, warnings).await?;
        Self::store_retryable_issues(&mut tx, &run_id, retryable_issues).await?;
        Self::store_source_code_issues(&mut tx, &run_id, source_code_issues).await?;

        tx.commit()
            .await
            .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;

        Ok(())
    }
}