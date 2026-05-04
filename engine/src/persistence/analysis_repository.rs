use sqlx::Postgres;
use crate::model::{Module, Relation, Symbol, ServiceError};

pub trait AnalysisRepository {
    async fn store_batch(
        &self,
        modules: &[Module],
        symbols: &[Symbol],
        relations: &[Relation],
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
}

impl AnalysisRepository for SqlxAnalysisRepository {
    async fn store_batch(
        &self,
        modules: &[Module],
        symbols: &[Symbol],
        relations: &[Relation],
    ) -> Result<(), ServiceError> {
        let mut tx = self.pool.begin()
            .await
            .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;

        Self::store_modules(&mut tx, modules).await?;
        Self::store_symbols(&mut tx, symbols).await?;
        Self::store_relations(&mut tx, relations).await?;

        tx.commit()
            .await
            .map_err(|e| ServiceError::DatabaseRequestFailed(e.to_string()))?;

        Ok(())
    }
}