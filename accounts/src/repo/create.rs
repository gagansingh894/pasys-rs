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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain;
    use crate::repo::AccountReader;
    use chrono::Timelike;

    #[sqlx::test(migrations = "../migrations/accounts")]
    async fn successfully_create_an_account_and_retrieve_from_the_database(pool: sqlx::PgPool) {
        // arrange
        let repo = PgAccountRepository {
            db: common::database::Database::from_pool(pool).await.unwrap(),
        };

        let account = Account {
            id: uuid::Uuid::new_v4(),
            name: "test account".to_string(),
            account_type: domain::account::Type::Customer,
            account_status: domain::account::Status::Active,
            created_by: "test creator".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // act - insert account
        let resp = repo.create_account(&account).await;
        assert!(resp.is_ok());

        // assert - fetch account
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
}
