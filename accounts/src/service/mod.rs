use crate::domain::account::{Account, Status, Type};
use crate::repo::AccountRepository;

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

    pub async fn create_account(
        &self,
        name: impl Into<String>,
        account_type: Type,
        created_by: impl Into<String>,
    ) -> anyhow::Result<Account> {
        let account = Account::new(name.into(), account_type, Status::Active, created_by.into());

        match self.repo.create_account(&account).await {
            Ok(account) => Ok(account),
            Err(e) => {
                anyhow::bail!("Failed to create_account: {:?}", e);
            }
        }
    }

    pub async fn get_account_by_id(&self, id: &str) -> anyhow::Result<Account> {
        match self.repo.get_account_by_id(id).await {
            Ok(account) => Ok(account),
            Err(e) => {
                anyhow::bail!("Failed to get_account_by_id: {:?}", e);
            }
        }
    }

    pub async fn get_accounts_by_type(&self, account_type: Type) -> anyhow::Result<Vec<Account>> {
        match self.repo.get_accounts_by_type(account_type).await {
            Ok(accounts) => Ok(accounts),
            Err(e) => {
                anyhow::bail!("Failed to get_accounts_by_type: {:?}", e);
            }
        }
    }

    pub async fn get_accounts_by_status(&self, status: Status) -> anyhow::Result<Vec<Account>> {
        match self.repo.get_accounts_by_status(status).await {
            Ok(accounts) => Ok(accounts),
            Err(e) => {
                anyhow::bail!("Failed to get_accounts_by_status: {:?}", e);
            }
        }
    }
}
