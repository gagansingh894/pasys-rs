use crate::domain::account::Account;
use crate::repo::{AccountWriter, PgAccountRepository};
use async_trait::async_trait;
use chrono;

#[async_trait]
impl AccountWriter for PgAccountRepository {
    async fn create_account(&self, account: &Account) -> anyhow::Result<Account> {
        let now = chrono::Utc::now();
        let result = sqlx::query_as::<_, Account>(
            r#"
            INSERT INTO accounts (id, name, account_type, account_status, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, name, account_type, account_status, created_by, created_at, updated_at
            "#
        )
        .bind(account.id)
            .bind(account.name.as_str())
            .bind(account.account_type.clone())
            .bind(account.account_status.clone())
            .bind(account.created_by.as_str())
            .bind(now)
            .bind(now)
            .fetch_one(&self.db.writer)
            .await;

        match result {
            Ok(account) => Ok(account),
            Err(e) => anyhow::bail!("Failed to insert into database: {e}"),
        }
    }
}
