use sqlx::SqlitePool;

/// Maintains all context for the directory service.
#[derive(Debug)]
pub struct DirectoryContext {
    pub host_address: String,
    pub database_pool: SqlitePool,
}
