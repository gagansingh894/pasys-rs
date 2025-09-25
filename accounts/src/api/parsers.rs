use crate::domain;
use accounts_proto::accounts_v1;

use prost_types::Timestamp;

pub fn parse_account_to_proto(account: domain::account::Account) -> accounts_v1::Account {
    accounts_v1::Account {
        id: account.id.to_string(),
        name: account.name.to_string(),
        r#type: match account.account_type {
            domain::account::Type::Customer => accounts_v1::AccountType::Customer as i32,
            domain::account::Type::Merchant => accounts_v1::AccountType::Merchant as i32,
            domain::account::Type::System => accounts_v1::AccountType::System as i32,
        },
        status: match account.account_status {
            domain::account::Status::Active => accounts_v1::AccountStatus::Active as i32,
            domain::account::Status::Frozen => accounts_v1::AccountStatus::Active as i32,
            domain::account::Status::Closed => accounts_v1::AccountStatus::Active as i32,
        },
        created_by: account.created_by,
        created_at: Some(Timestamp {
            seconds: account.created_at.timestamp(),
            nanos: account.created_at.timestamp_subsec_nanos() as i32,
        }),
        updated_at: Some(Timestamp {
            seconds: account.updated_at.timestamp(),
            nanos: account.updated_at.timestamp_subsec_nanos() as i32,
        }),
    }
}

pub fn parse_to_domain_account_type(account_type: i32) -> anyhow::Result<domain::account::Type> {
    match account_type {
        0 => Err(anyhow::anyhow!("Unspecified account type")),
        1 => Ok(domain::account::Type::Customer),
        2 => Ok(domain::account::Type::Merchant),
        3 => Ok(domain::account::Type::System),
        _ => Err(anyhow::anyhow!("Unspecified account type")),
    }
}
