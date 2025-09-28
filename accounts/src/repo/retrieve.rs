use crate::domain::account::{Account, Status, Type};
use crate::repo::{AccountReader, PgAccountRepository};
use anyhow;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
impl AccountReader for PgAccountRepository {
    async fn get_account_by_id(&self, id: &str) -> anyhow::Result<Account> {
        let account_id = match Uuid::parse_str(id) {
            Ok(account_id) => account_id,
            Err(e) => anyhow::bail!("failed to parse account_id to str: {}", e),
        };

        let result = sqlx::query_as::<_, Account>(
            r#"
                SELECT id, name, account_type, account_status, created_by, created_at, updated_at
                FROM accounts
                WHERE id = $1
                "#,
        )
        .bind(account_id)
        .fetch_one(&self.db.reader)
        .await;

        match result {
            Ok(account) => Ok(account),
            Err(sqlx::Error::RowNotFound) => {
                tracing::error!("Account {id} not found");
                anyhow::bail!("Account {id} not found")
            }
            Err(e) => {
                tracing::error!("Failed to get_account_by_id: {e}");
                anyhow::bail!("Failed to get_account_by_id: {e}")
            }
        }
    }

    async fn get_accounts_by_type(&self, account_type: Type) -> anyhow::Result<Vec<Account>> {
        let result = sqlx::query_as::<_, Account>(
            r#"
                SELECT id, name, account_type, account_status, created_by, created_at, updated_at
                FROM accounts
                WHERE account_type = $1
                "#,
        )
        .bind(account_type.clone())
        .fetch_all(&self.db.reader)
        .await;

        match result {
            Ok(accounts) => Ok(accounts),
            Err(sqlx::Error::RowNotFound) => {
                tracing::error!("No accounts found with type: {:?}", account_type.as_ref());
                anyhow::bail!("No accounts found with type: {:?}", account_type.as_ref())
            }
            Err(e) => {
                tracing::error!("Failed to get_accounts_by_type: {e}");
                anyhow::bail!("Failed to get_accounts_by_type: {e}")
            }
        }
    }

    async fn get_accounts_by_status(&self, status: Status) -> anyhow::Result<Vec<Account>> {
        let result = sqlx::query_as::<_, Account>(
            r#"
                SELECT id, name, account_type, account_status, created_by, created_at, updated_at
                FROM accounts
                WHERE account_status = $1
                "#,
        )
        .bind(status.clone())
        .fetch_all(&self.db.reader)
        .await;

        match result {
            Ok(accounts) => Ok(accounts),
            Err(sqlx::Error::RowNotFound) => {
                tracing::error!("No accounts found with status: {:?}", status.as_ref());
                anyhow::bail!("No accounts found with status: {:?}", status.as_ref())
            }
            Err(e) => {
                tracing::error!("Failed to get_accounts_by_status: {e}");
                anyhow::bail!("Failed to get_accounts_by_status: {e}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain;
    use crate::repo::AccountWriter;
    use chrono::Timelike;

    #[sqlx::test(migrations = "../migrations/accounts")]
    async fn successfully_retrieve_account_by_id_from_database(pool: sqlx::PgPool) {
        // arrange - setup repo, insert account
        let repo =
            PgAccountRepository::new(common::database::Database::from_pool(pool).await.unwrap());

        let account = Account {
            id: uuid::Uuid::new_v4(),
            name: "test account".to_string(),
            account_type: domain::account::Type::Customer,
            account_status: domain::account::Status::Active,
            created_by: "test creator".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let resp = repo.create_account(&account).await;
        assert!(resp.is_ok());

        // act - fetch account
        let fetched_account = repo
            .get_account_by_id(account.id.to_string().as_str())
            .await;
        assert!(fetched_account.is_ok());

        // assert - compare inserted and fetched account
        let fetched_account = fetched_account.unwrap();
        assert_eq!(fetched_account.id, account.id);
        assert_eq!(fetched_account.name, account.name);
        assert_eq!(
            fetched_account.account_type.as_ref(),
            account.account_type.as_ref()
        );
        assert_eq!(
            fetched_account.account_status.as_ref(),
            account.account_status.as_ref()
        );
        assert_eq!(fetched_account.created_by, account.created_by);
        assert_eq!(
            fetched_account.created_at.with_nanosecond(0).unwrap(),
            account.created_at.with_nanosecond(0).unwrap()
        );
        assert_eq!(
            fetched_account.updated_at.with_nanosecond(0).unwrap(),
            account.updated_at.with_nanosecond(0).unwrap()
        );
    }

    #[sqlx::test(migrations = "../migrations/accounts")]
    async fn successfully_retrieve_accounts_by_status_from_database(pool: sqlx::PgPool) {
        // arrange - setup repo, insert accounts
        let repo =
            PgAccountRepository::new(common::database::Database::from_pool(pool).await.unwrap());

        let accounts: Vec<Account> = vec![
            Account {
                id: uuid::Uuid::new_v4(),
                name: "test account 1".to_string(),
                account_type: domain::account::Type::Customer,
                account_status: domain::account::Status::Closed,
                created_by: "test creator 1 ".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            Account {
                id: uuid::Uuid::new_v4(),
                name: "test account 2".to_string(),
                account_type: domain::account::Type::Customer,
                account_status: domain::account::Status::Active,
                created_by: "test creator 2".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            Account {
                id: uuid::Uuid::new_v4(),
                name: "test account 3".to_string(),
                account_type: domain::account::Type::Merchant,
                account_status: domain::account::Status::Active,
                created_by: "test creator 3".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
        ];

        for account in &accounts {
            let resp = repo.create_account(account).await;
            assert!(resp.is_ok());
        }

        // act - fetch account
        let fetched_accounts = repo
            .get_accounts_by_status(domain::account::Status::Active)
            .await;
        assert!(fetched_accounts.is_ok());

        // assert - compare inserted and fetched account
        let fetched_accounts = fetched_accounts.unwrap();
        assert_eq!(fetched_accounts.len(), 2);

        // todo: add assertions on account level
    }

    #[sqlx::test(migrations = "../migrations/accounts")]
    async fn successfully_retrieve_accounts_by_type_from_database(pool: sqlx::PgPool) {
        // arrange - setup repo, insert accounts
        let repo =
            PgAccountRepository::new(common::database::Database::from_pool(pool).await.unwrap());

        let accounts: Vec<Account> = vec![
            Account {
                id: uuid::Uuid::new_v4(),
                name: "test account 1".to_string(),
                account_type: domain::account::Type::Customer,
                account_status: domain::account::Status::Active,
                created_by: "test creator 1 ".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            Account {
                id: uuid::Uuid::new_v4(),
                name: "test account 2".to_string(),
                account_type: domain::account::Type::Customer,
                account_status: domain::account::Status::Active,
                created_by: "test creator 2".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            Account {
                id: uuid::Uuid::new_v4(),
                name: "test account 3".to_string(),
                account_type: domain::account::Type::Merchant,
                account_status: domain::account::Status::Active,
                created_by: "test creator 3".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            Account {
                id: uuid::Uuid::new_v4(),
                name: "test account 4".to_string(),
                account_type: domain::account::Type::Merchant,
                account_status: domain::account::Status::Active,
                created_by: "test creator 4".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
        ];

        for account in &accounts {
            let resp = repo.create_account(account).await;
            assert!(resp.is_ok());
        }

        // act - fetch accounts
        let fetched_accounts = repo
            .get_accounts_by_type(domain::account::Type::Customer)
            .await;
        assert!(fetched_accounts.is_ok());

        // assert - compare inserted and fetched account
        let fetched_accounts = fetched_accounts.unwrap();
        assert_eq!(fetched_accounts.len(), 2);

        // todo: add assertions on account level
    }
}
