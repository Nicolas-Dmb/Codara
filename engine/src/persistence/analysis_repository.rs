use sqlx::Postgres;
use crate::model::{AnalysisWarning, Module, Relation, RetryableIssue, ServiceError, SourceCodeIssue, Symbol};

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
        let ids: Vec<_> = modules.iter().map(|m| m.id.to_string()).collect();
        let run_ids: Vec<_> = modules.iter().map(|m| m.run_id.to_string()).collect();
        let relative_paths: Vec<_> = modules.iter().map(|m| &m.relative_path).collect();
        let names: Vec<_> = modules.iter().map(|m| &m.name).collect();

        sqlx::query(
            r#"INSERT INTO module (id, run_id, relative_path, name) SELECT * FROM UNNEST(
                    $1::text[],
                    $2::text[],
                    $3::text[],
                    $4::text[]
                )"#
            )
            .bind(&ids)
            .bind(&run_ids)
            .bind(&relative_paths)
            .bind(&names)
            .execute(&mut **tx)
            .await
            .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;

        Ok(())
    }

    async fn store_symbols(
        tx: &mut sqlx::Transaction<'_, Postgres>,
        symbols: &[Symbol],
    ) -> Result<(), ServiceError> {
        let ids: Vec<String> = symbols.iter().map(|s| s.id.to_string()).collect();
        let module_ids: Vec<String> = symbols.iter().map(|s| s.module_id.to_string()).collect();
        let run_ids: Vec<String> = symbols.iter().map(|s| s.run_id.to_string()).collect();
        let parent_symbol_ids: Vec<Option<String>> = symbols
            .iter()
            .map(|s| s.parent_symbol_id.as_ref().map(|id| id.to_string()))
            .collect();
        let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
        let kinds: Vec<String> = symbols.iter().map(|s| s.kind.to_string()).collect();
        let docs: Vec<&str> = symbols.iter().map(|s| s.doc.as_str()).collect();
        let locations: Vec<&str> = symbols.iter().map(|s| s.location.as_str()).collect();
        let start_lines: Vec<i64> = symbols.iter().map(|s| s.start_line as i64).collect();
        let end_lines: Vec<i64> = symbols.iter().map(|s| s.end_line as i64).collect();

        sqlx::query(
            "INSERT INTO symbol (id, module_id, run_id, parent_symbol_id, name, kind, doc, location, start_line, end_line) SELECT * FROM UNNEST(
                $1::text[],
                $2::text[],
                $3::text[],
                $4::text[],
                $5::text[],
                $6::text[],
                $7::text[],
                $8::text[],
                $9::int8[],
                $10::int8[]
            )"
        )
        .bind(&ids)
        .bind(&module_ids)
        .bind(&run_ids)
        .bind(&parent_symbol_ids)
        .bind(&names)
        .bind(&kinds)
        .bind(&docs)
        .bind(&locations)
        .bind(&start_lines)
        .bind(&end_lines)
        .execute(&mut **tx)
        .await
        .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;

        Ok(())
    }

    async fn store_relations(
        tx: &mut sqlx::Transaction<'_, Postgres>,
        relations: &[Relation],
    ) -> Result<(), ServiceError> {
        let ids: Vec<String> = relations.iter().map(|r| r.id.to_string()).collect();
        let module_ids: Vec<String> = relations.iter().map(|r| r.module_id.to_string()).collect();
        let run_ids: Vec<String> = relations.iter().map(|r| r.run_id.to_string()).collect();
        let parent_symbol_ids: Vec<Option<String>> = relations
            .iter()
            .map(|r| r.parent_symbol_id.as_ref().map(|id| id.to_string()))
            .collect();
        let imported_names: Vec<&str> = relations.iter().map(|r| r.imported_name.as_str()).collect();
        let source_paths: Vec<&str> = relations.iter().map(|r| r.source_path.as_str()).collect();
        let target_symbol_ids: Vec<Option<String>> = relations
            .iter()
            .map(|r| r.target_symbol_id.as_ref().map(|id| id.to_string()))
            .collect();
        let kinds: Vec<String> = relations.iter().map(|r| r.kind.to_string()).collect();
        let lines: Vec<i64> = relations.iter().map(|r| r.line as i64).collect();

        sqlx::query(
            "INSERT INTO relation (id, module_id, run_id, parent_symbol_id, imported_name, source_path, target_symbol_id, kind, line) SELECT * FROM UNNEST(
                $1::text[],
                $2::text[],
                $3::text[],
                $4::text[],
                $5::text[],
                $6::text[],
                $7::text[],
                $8::text[],
                $9::int8[]
            )"
        )
        .bind(&ids)
        .bind(&module_ids)
        .bind(&run_ids)
        .bind(&parent_symbol_ids)
        .bind(&imported_names)
        .bind(&source_paths)
        .bind(&target_symbol_ids)
        .bind(&kinds)
        .bind(&lines)
        .execute(&mut **tx)
        .await
        .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;

        Ok(())
    }

    async fn store_warnings(
        tx: &mut sqlx::Transaction<'_, Postgres>,
        run_id: &str,
        warnings: &[AnalysisWarning],
    ) -> Result<(), ServiceError> {
        let run_ids: Vec<&str> = warnings.iter().map(|_| run_id).collect();
        let kinds: Vec<&str> = warnings
            .iter()
            .map(|w| match w {
                AnalysisWarning::UnsupportedFileType { .. } => "unsupported_file_type",
                AnalysisWarning::IgnoredFile { .. } => "ignored_file",
            })
            .collect();
        let paths: Vec<&str> = warnings
            .iter()
            .map(|w| match w {
                AnalysisWarning::UnsupportedFileType { path } => path.as_str(),
                AnalysisWarning::IgnoredFile { path } => path.as_str(),
            })
            .collect();

        sqlx::query(
            "INSERT INTO analysis_warning (run_id, kind, path) SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[])"
        )
        .bind(&run_ids)
        .bind(&kinds)
        .bind(&paths)
        .execute(&mut **tx)
        .await
        .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;

        Ok(())
    }

    async fn store_retryable_issues(
        tx: &mut sqlx::Transaction<'_, Postgres>,
        run_id: &str,
        issues: &[RetryableIssue],
    ) -> Result<(), ServiceError> {
        let run_ids: Vec<&str> = issues.iter().map(|_| run_id).collect();
        let kinds: Vec<&str> = issues
            .iter()
            .map(|i| match i {
                RetryableIssue::UnreadableDirectory { .. } => "unreadable_directory",
                RetryableIssue::UnreadableFile { .. } => "unreadable_file",
                RetryableIssue::AdapterFailed { .. } => "adapter_failed",
                RetryableIssue::UnresolvedImport { .. } => "unresolved_import",
            })
            .collect();
        let paths: Vec<&str> = issues
            .iter()
            .map(|i| match i {
                RetryableIssue::UnreadableDirectory { path, .. } => path.as_str(),
                RetryableIssue::UnreadableFile { path, .. } => path.as_str(),
                RetryableIssue::AdapterFailed { path, .. } => path.as_str(),
                RetryableIssue::UnresolvedImport { path, .. } => path.as_str(),
            })
            .collect();
        let reasons: Vec<&str> = issues
            .iter()
            .map(|i| match i {
                RetryableIssue::UnreadableDirectory { reason, .. } => reason.as_str(),
                RetryableIssue::UnreadableFile { reason, .. } => reason.as_str(),
                RetryableIssue::AdapterFailed { reason, .. } => reason.as_str(),
                RetryableIssue::UnresolvedImport { import_name, .. } => import_name.as_str(),
            })
            .collect();

        sqlx::query(
            "INSERT INTO retryable_issue (run_id, kind, path, reason) SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[])"
        )
        .bind(&run_ids)
        .bind(&kinds)
        .bind(&paths)
        .bind(&reasons)
        .execute(&mut **tx)
        .await
        .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;

        Ok(())
    }

    async fn store_source_code_issues(
        tx: &mut sqlx::Transaction<'_, Postgres>,
        run_id: &str,
        issues: &[SourceCodeIssue],
    ) -> Result<(), ServiceError> {
        let run_ids: Vec<&str> = issues.iter().map(|_| run_id).collect();
        let kinds: Vec<&str> = issues
            .iter()
            .map(|i| match i {
                SourceCodeIssue::InvalidSyntax { .. } => "invalid_syntax",
            })
            .collect();
        let paths: Vec<&str> = issues
            .iter()
            .map(|i| match i {
                SourceCodeIssue::InvalidSyntax { path, .. } => path.as_str(),
            })
            .collect();
        let reasons: Vec<&str> = issues
            .iter()
            .map(|i| match i {
                SourceCodeIssue::InvalidSyntax { reason, .. } => reason.as_str(),
            })
            .collect();

        sqlx::query(
            "INSERT INTO source_code_issue (run_id, kind, path, reason) SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[])"
        )
        .bind(&run_ids)
        .bind(&kinds)
        .bind(&paths)
        .bind(&reasons)
        .execute(&mut **tx)
        .await
        .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;

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