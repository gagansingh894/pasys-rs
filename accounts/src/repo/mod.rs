use crate::domain::account::{Account, Status, Type};
use async_trait::async_trait;
use common::database::Database;

mod create;
mod retrieve;

#[derive(Clone, Debug)]
pub struct PgAccountRepository {
    pub db: Database,
}

impl PgAccountRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
pub trait AccountRepository: AccountReader + AccountWriter + 'static + Sync + Send {}

#[async_trait]
pub trait AccountReader: 'static + Sync + Send {
    async fn get_account_by_id(&self, id: &str) -> anyhow::Result<Account>;
    async fn get_accounts_by_type(&self, account_type: Type) -> anyhow::Result<Vec<Account>>;
    async fn get_accounts_by_status(&self, status: Status) -> anyhow::Result<Vec<Account>>;
}

#[async_trait]
pub trait AccountWriter: 'static + Sync + Send {
    async fn create_account(&self, account: &Account) -> anyhow::Result<Account>;
}

impl AccountRepository for PgAccountRepository {}
