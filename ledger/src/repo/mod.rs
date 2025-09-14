use async_trait::async_trait;
use common::database::Database;

#[derive(Debug, Clone)]
pub struct PgLedgerRepository {
    db: Database,
}

impl PgLedgerRepository {
    pub fn new(db: Database) -> impl LedgerRepository {
        Self { db }
    }
}

#[async_trait]
pub trait LedgerRepository: LedgerWriter + LedgerReader + 'static + Send + Sync {}

#[async_trait]
pub trait LedgerWriter: 'static + Send + Sync {}

#[async_trait]
pub trait LedgerReader: 'static + Send + Sync {}

impl LedgerRepository for PgLedgerRepository {}

#[async_trait]
impl LedgerReader for PgLedgerRepository {}

#[async_trait]
impl LedgerWriter for PgLedgerRepository {}
