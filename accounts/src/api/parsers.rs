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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain;
    use crate::domain::account::{Account, Status, Type};
    use accounts_proto::accounts_v1::AccountType;

    #[test]
    fn test_parse_to_domain_account_type() {
        #[derive(Debug)]
        struct TestCase {
            name: &'static str,
            account_type: AccountType,
            expected: Option<domain::account::Type>,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                name: "successfully parse to domain account type when input account type is customer",
                account_type: AccountType::Customer,
                expected: Some(domain::account::Type::Customer),
            },
            TestCase {
                name: "successfully parse to domain account type when input account type is merchant",
                account_type: AccountType::Merchant,
                expected: Some(domain::account::Type::Merchant),
            },
            TestCase {
                name: "successfully parse to domain account type when input account type is system",
                account_type: AccountType::System,
                expected: Some(domain::account::Type::System),
            },
            TestCase {
                name: "error when input account type is unspecified",
                account_type: AccountType::Unspecified,
                expected: None,
            },
        ];

        for test_case in test_cases {
            let resp = parse_to_domain_account_type(test_case.account_type as i32);
            match test_case.expected {
                Some(expected) => {
                    assert!(resp.is_ok(), "{}", test_case.name);
                    assert_eq!(resp.unwrap(), expected, "{}", test_case.name);
                }
                None => assert!(resp.is_err(), "{}", test_case.name),
            }
        }
    }

    #[test]
    fn successfully_parse_account_to_proto() {
        // arrange
        let account = Account {
            id: uuid::Uuid::new_v4(),
            name: "test".to_string(),
            account_type: Type::Customer,
            account_status: Status::Active,
            created_by: "test".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // act
        let account_proto = parse_account_to_proto(account.clone());

        // assert
        assert_eq!(account_proto.id, account.id.to_string().as_str());
        assert_eq!(account_proto.name, account.name);
        assert_eq!(account_proto.r#type, AccountType::Customer as i32);
        assert_eq!(account_proto.status, Status::Active as i32);
    }
}
