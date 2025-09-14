use crate::domain::account::{Account, Status, Type};
use crate::repo::AccountRepository;
use std::sync::Arc;

pub struct AccountsService<R>
where
    R: AccountRepository,
{
    repo: R,
}

impl<R> AccountsService<R>
where
    R: AccountRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_account(account: &Account) -> anyhow::Result<Account> {
        todo!()
    }

    pub async fn get_account_by_id(&self, id: &str) -> anyhow::Result<Account> {
        todo!()
    }

    pub async fn get_accounts_by_type(&self, account_type: Type) -> anyhow::Result<Vec<Account>> {
        todo!()
    }

    pub async fn get_accounts_by_status(&self, status: Status) -> anyhow::Result<Vec<Account>> {
        todo!()
    }
}
