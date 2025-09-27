#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "account_status", rename_all = "lowercase")]
pub enum Status {
    Active,
    Frozen,
    Closed,
}

impl AsRef<str> for Status {
    fn as_ref(&self) -> &str {
        match self {
            Status::Active => "active",
            Status::Frozen => "frozen",
            Status::Closed => "closed",
        }
    }
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "account_type", rename_all = "lowercase")]
pub enum Type {
    Customer,
    Merchant,
    System,
}

impl AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        match self {
            Type::Customer => "customer",
            Type::Merchant => "merchant",
            Type::System => "system",
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Account {
    pub id: uuid::Uuid,
    pub name: String,
    pub account_type: Type,
    pub account_status: Status,
    pub created_by: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Account {
    pub fn new(
        name: impl Into<String>,
        account_type: Type,
        account_status: Status,
        created_by: impl Into<String>,
    ) -> Self {
        Account {
            id: uuid::Uuid::new_v4(),
            name: name.into(),
            account_type,
            account_status,
            created_by: created_by.into(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    pub fn set_created_at(&mut self, created_at: chrono::DateTime<chrono::Utc>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: chrono::DateTime<chrono::Utc>) {
        self.updated_at = updated_at;
    }

    pub fn set_status(&mut self, status: Status) {
        self.account_status = status;
    }
}
