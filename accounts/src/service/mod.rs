use crate::domain::account::{Account, Status, Type};
use crate::repo::PgAccountRepository;

#[derive(Clone, Debug)]
pub struct AccountsService {
    repo: PgAccountRepository,
}

impl AccountsService {
    pub fn new(repo: PgAccountRepository) -> Self {
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
