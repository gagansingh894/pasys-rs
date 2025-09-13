use crate::domain::account::Account;
use crate::repo::{AccountWriter, PgAccountRepository};
use async_trait::async_trait;

#[async_trait]
impl AccountWriter for PgAccountRepository {
    async fn create_account(&self, account: &Account) -> anyhow::Result<Account> {
        todo!()
    }
}
