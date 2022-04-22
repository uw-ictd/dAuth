use sqlx::SqlitePool;

pub struct DirectoryContext {
    pub address: String,
    pub database_pool: SqlitePool,
}
