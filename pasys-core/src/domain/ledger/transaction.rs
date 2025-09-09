#[derive(Debug, Clone)]
pub enum Status {
    Init,
    Pending,
    Success,
    Failed,
    Fraud,
    Refund,
    Refunded
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: uuid::Uuid,
    pub client_id: uuid::Uuid,
    pub debit_account_id: uuid::Uuid,
    pub credit_account_id: uuid::Uuid,
    pub amount_minor: i64,
    pub currency: String,
    pub status: Status,
    pub idempotency_key: String,
    pub request_timestamp: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}