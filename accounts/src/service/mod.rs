use crate::domain::account::{Account, Status, Type};
use crate::repo::AccountRepository;

#[derive(Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo;
    use crate::repo::PgAccountRepository;
    use common::database;

    async fn setup_database(repo: &impl AccountRepository) -> anyhow::Result<()> {
        let accounts: Vec<Account> = vec![
            Account {
                id: uuid::Uuid::new_v4(),
                name: "test account 1".to_string(),
                account_type: Type::Customer,
                account_status: Status::Closed,
                created_by: "test creator 1".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            Account {
                id: uuid::Uuid::new_v4(),
                name: "test account 2".to_string(),
                account_type: Type::Customer,
                account_status: Status::Active,
                created_by: "test creator 2".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            Account {
                id: uuid::Uuid::new_v4(),
                name: "test account 3".to_string(),
                account_type: Type::Merchant,
                account_status: Status::Active,
                created_by: "test creator 3".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
        ];

        for account in &accounts {
            let resp = repo.create_account(account).await;
            assert!(resp.is_ok());
        }

        Ok(())
    }

    #[sqlx::test(migrations = "../migrations/accounts")]
    async fn successfully_retrieve_account_by_id(pool: sqlx::PgPool) {
        // arrange
        let repo = repo::PgAccountRepository {
            db: database::Database::from_pool(pool).await.unwrap(),
        };
        let account_service = AccountsService::new(repo);

        // act - we are also testing the create account here as well
        let result = account_service
            .create_account("test account", Type::Customer, "test user")
            .await;
        assert!(result.is_ok());

        // assert
        let account = result.unwrap();
        let fetched_account = account_service
            .get_account_by_id(account.id.to_string().as_str())
            .await;
        assert!(fetched_account.is_ok());

        assert_eq!(account.id, fetched_account.unwrap().id);
    }

    #[sqlx::test(migrations = "../migrations/accounts")]
    async fn successfully_retrieve_accounts_by_type(pool: sqlx::PgPool) {
        // arrange
        let repo =
            PgAccountRepository::new(common::database::Database::from_pool(pool).await.unwrap());
        let account_service = AccountsService::new(repo.clone());
        setup_database(&repo).await.unwrap();

        // act
        let result = account_service.get_accounts_by_type(Type::Merchant).await;
        assert!(result.is_ok());

        // assert
        let accounts = result.unwrap();
        assert_eq!(accounts.len(), 1);
    }

    #[sqlx::test(migrations = "../migrations/accounts")]
    async fn successfully_retrieve_accounts_by_status(pool: sqlx::PgPool) {
        // arrange
        let repo = repo::PgAccountRepository {
            db: database::Database::from_pool(pool).await.unwrap(),
        };
        let account_service = AccountsService::new(repo.clone());
        setup_database(&repo).await.unwrap();

        // act
        let result = account_service.get_accounts_by_status(Status::Closed).await;
        assert!(result.is_ok());

        // assert
        let accounts = result.unwrap();
        assert_eq!(accounts.len(), 1);
    }
}
