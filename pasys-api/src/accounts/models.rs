use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateAccountRequest {
    pub name: String,
    pub account_type: String,
    pub created_by: String,
}

#[derive(Serialize)]
pub struct CreateAccountResponse {
    pub account: Option<Account>,
}

#[derive(Serialize)]
pub struct GetAccountResponse {
    pub account: Option<Account>,
}

#[derive(Serialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub account_type: String,
    pub account_status: String,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
