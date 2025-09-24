use crate::domain::account::{Account, Status, Type};
use crate::repo::{AccountReader, AccountRepository, PgAccountRepository};
use anyhow;
use async_trait::async_trait;

#[async_trait]
impl AccountReader for PgAccountRepository {
    async fn get_account_by_id(&self, id: &str) -> anyhow::Result<Account> {
        let result = sqlx::query_as::<_, Account>(
            r#"
                SELECT id, name, account_type, account_status, created_by, created_at, updated_at
                FROM accounts
                WHERE id = $1
                "#,
        )
        .bind(id)
        .fetch_one(&self.db.reader)
        .await;

        match result {
            Ok(account) => Ok(account),
            Err(sqlx::Error::RowNotFound) => {
                anyhow::bail!("Account {id} not found")
            }
            Err(e) => {
                anyhow::bail!("Failed to get_account_by_id: {e}")
            }
        }
    }

    async fn get_accounts_by_type(&self, account_type: Type) -> anyhow::Result<Vec<Account>> {
        let result = sqlx::query_as::<_, Account>(
            r#"
                SELECT id, name, account_type, account_status, created_by, created_at, updated_at
                FROM accounts
                WHERE account_type = ?
                "#,
        )
        .bind(account_type.as_ref())
        .fetch_all(&self.db.reader)
        .await;

        match result {
            Ok(accounts) => Ok(accounts),
            Err(sqlx::Error::RowNotFound) => {
                anyhow::bail!("No accounts found with type: {:?}", account_type.as_ref())
            }
            Err(e) => {
                anyhow::bail!("Failed to get_accounts_by_type: {e}")
            }
        }
    }

    async fn get_accounts_by_status(&self, status: Status) -> anyhow::Result<Vec<Account>> {
        let result = sqlx::query_as::<_, Account>(
            r#"
                SELECT id, name, account_type, account_status, created_by, created_at, updated_at
                FROM accounts
                WHERE account_status = ?
                "#,
        )
        .bind(status.as_ref())
        .fetch_all(&self.db.reader)
        .await;

        match result {
            Ok(accounts) => Ok(accounts),
            Err(sqlx::Error::RowNotFound) => {
                anyhow::bail!("No accounts found with status: {:?}", status.as_ref())
            }
            Err(e) => {
                anyhow::bail!("Failed to get_accounts_by_status: {e}")
            }
        }
    }
}
