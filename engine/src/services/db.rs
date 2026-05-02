use sqlx::postgres::PgPoolOptions;
use crate::model::ServiceError;

// Max number of connections could be set to 5 for now, but it can be changed later if needed
const MAX_CONNECTIONS: u32 = 5;

pub trait Db {
    fn post(&self, query: &str) -> Result<(), ServiceError>;
}

pub struct SqlxDb {
    pub pool: sqlx::PgPool,
}


impl SqlxDb {
    pub async fn init_db(db_url: &str) -> Result<Self, ServiceError> {
        let pool = PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(db_url).await.map_err(|_| ServiceError::DatabaseInitializationFailed)?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await.map_err(|_| ServiceError::MigrationFailed("Failed to run migrations".into()))?;
        
        Ok(Self { pool })
    }
}

impl Db for SqlxDb {
    fn post(&self, query: &str) -> Result<(), ServiceError> {
            unimplemented!();
    }
}