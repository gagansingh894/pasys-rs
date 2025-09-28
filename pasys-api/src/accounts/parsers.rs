use crate::accounts::models;

use accounts_proto::accounts_v1;

pub(crate) fn parse_account_proto(
    account: accounts_v1::Account,
) -> anyhow::Result<models::Account> {
    let account_type = match account.r#type {
        1 => accounts_v1::AccountType::Customer.as_str_name(),
        2 => accounts_v1::AccountType::Merchant.as_str_name(),
        3 => accounts_v1::AccountType::System.as_str_name(),
        _ => anyhow::bail!("Unsupported account type"),
    };

    let account_status = match account.status {
        1 => accounts_v1::AccountStatus::Active.as_str_name(),
        2 => accounts_v1::AccountStatus::Frozen.as_str_name(),
        3 => accounts_v1::AccountStatus::Closed.as_str_name(),
        _ => anyhow::bail!("Unsupported account status"),
    };

    let created_at = match account.created_at {
        Some(created_at) => created_at.to_string(),
        None => "".to_string(),
    };
    let updated_at = match account.updated_at {
        Some(updated_at) => updated_at.to_string(),
        None => "".to_string(),
    };

    Ok(models::Account {
        id: account.id,
        name: account.name,
        account_type: account_type.to_string(),
        account_status: account_status.to_string(),
        created_by: account.created_by,
        created_at,
        updated_at,
    })
}
