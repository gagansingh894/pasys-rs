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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::SecondsFormat;

    #[test]
    fn test_parse_account_proto() {
        struct TestCase {
            name: &'static str,
            account_proto: accounts_v1::Account,
            expected: Option<models::Account>,
        }

        let account_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now();

        let tests: Vec<TestCase> = vec![
            TestCase {
                name: "fails to parse account proto due to invalid account type",
                account_proto: accounts_v1::Account {
                    id: account_id.clone(),
                    name: "test".to_string(),
                    r#type: 0,
                    status: 0,
                    created_by: "test".to_string(),
                    created_at: Some(prost_types::Timestamp {
                        seconds: now.timestamp(),
                        nanos: now.timestamp_subsec_nanos() as i32,
                    }),
                    updated_at: Some(prost_types::Timestamp {
                        seconds: now.timestamp(),
                        nanos: now.timestamp_subsec_nanos() as i32,
                    }),
                },
                expected: None {},
            },
            TestCase {
                name: "fails to parse account proto due to invalid account status",
                account_proto: accounts_v1::Account {
                    id: account_id.clone(),
                    name: "test".to_string(),
                    r#type: 1,
                    status: 0,
                    created_by: "test".to_string(),
                    created_at: Some(prost_types::Timestamp {
                        seconds: now.timestamp(),
                        nanos: now.timestamp_subsec_nanos() as i32,
                    }),
                    updated_at: Some(prost_types::Timestamp {
                        seconds: now.timestamp(),
                        nanos: now.timestamp_subsec_nanos() as i32,
                    }),
                },
                expected: None,
            },
            TestCase {
                name: "successfully parse account proto",
                account_proto: accounts_v1::Account {
                    id: account_id.clone(),
                    name: "test".to_string(),
                    r#type: 1,
                    status: 1,
                    created_by: "test".to_string(),
                    created_at: Some(prost_types::Timestamp {
                        seconds: now.timestamp(),
                        nanos: now.timestamp_subsec_nanos() as i32,
                    }),
                    updated_at: Some(prost_types::Timestamp {
                        seconds: now.timestamp(),
                        nanos: now.timestamp_subsec_nanos() as i32,
                    }),
                },
                expected: Some(models::Account {
                    id: account_id.to_string(),
                    name: "test".to_string(),
                    account_type: "ACCOUNT_TYPE_CUSTOMER".to_string(),
                    account_status: "ACCOUNT_STATUS_ACTIVE".to_string(),
                    created_by: "test".to_string(),
                    created_at: now.to_rfc3339_opts(SecondsFormat::Micros, true),
                    updated_at: now.to_rfc3339_opts(SecondsFormat::Micros, true),
                }),
            },
        ];

        for test_case in tests {
            let resp = parse_account_proto(test_case.account_proto);
            match test_case.expected {
                None => {}
                Some(expected) => {
                    assert!(resp.is_ok(), "{}", test_case.name);
                    assert_eq!(resp.unwrap(), expected, "{}", test_case.name)
                }
            }
        }
    }
}
