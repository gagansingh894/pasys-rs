use crate::domain::account::{Account, Status, Type};
use crate::repo::{AccountReader, AccountRepository, PgAccountRepository};
use anyhow;
use async_trait::async_trait;

#[async_trait]
impl AccountReader for PgAccountRepository {
    async fn get_account_by_id(&self, id: &str) -> anyhow::Result<Account> {
        todo!()
    }

    async fn get_accounts_by_type(&self, account_type: Type) -> anyhow::Result<Vec<Account>> {
        todo!()
    }

    async fn get_accounts_by_status(&self, status: Status) -> anyhow::Result<Vec<Account>> {
        todo!()
    }
}
