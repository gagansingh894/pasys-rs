
#[derive(Debug, Clone)]
pub enum Status {
    Active,
    Frozen,
    Closed
}

#[derive(Debug, Clone)]
pub enum Type {
    Customer,
    Merchant,
    System
}

#[derive(Debug,Clone)]
pub struct Account {
    pub id: uuid::Uuid,
    pub name: String,
    pub account_type: Type,
    pub status: Status,
    pub created_by: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Account {
    pub fn new(name: impl Into<String>, account_type: Type, account_status: Status, created_by: impl Into<String>) -> Self {
        Account {
            id: uuid::Uuid::new_v4(),
            name: name.into(),
            account_type,
            status: account_status,
            created_by: created_by.into(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now()
        }
    }

    pub fn set_created_at(&mut self, created_at: chrono::DateTime<chrono::Utc>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: chrono::DateTime<chrono::Utc>) {
        self.updated_at = updated_at;
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}
