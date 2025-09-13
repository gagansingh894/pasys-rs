#[derive(Debug, Clone)]
pub enum Type {
    Credit,
    Debit,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub id: uuid::Uuid,
    pub transaction_id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub entry_type: Type,
    pub amount_minor: i64,
    pub currency: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
